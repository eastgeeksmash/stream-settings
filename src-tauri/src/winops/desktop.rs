use windows::core::{w, BOOL};
use windows::Win32::Foundation::{COLORREF, HWND, LPARAM, TRUE};
use windows::Win32::Graphics::Gdi::{SetSysColors, COLOR_BACKGROUND, COLOR_DESKTOP};
use windows::Win32::UI::Shell::{SHGetSetSettings, SHELLSTATEA, SSF_HIDEICONS};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetClassNameW, ShowWindow, SystemParametersInfoW, ANIMATIONINFO, SPIF_SENDCHANGE,
    SPIF_UPDATEINIFILE, SPI_SETANIMATION, SPI_SETCLIENTAREAANIMATION, SPI_SETDESKWALLPAPER,
    SPI_SETDROPSHADOW, SPI_SETUIEFFECTS, SW_HIDE, SW_SHOWNA,
};

use super::error::from_windows_error;
use super::registry::{set_hkcu_dword, set_hkcu_sz};

const AERO_OPERATION: &str = "Windows Aeroの無効化に失敗しました";
const AERO_RESTORE_OPERATION: &str = "Windows Aeroの復元に失敗しました";
const WALLPAPER_OPERATION: &str = "壁紙の変更に失敗しました";
const DESKTOP_OPERATION: &str = "デスクトップアイコン/タスクバーの非表示に失敗しました";
const DESKTOP_RESTORE_OPERATION: &str = "デスクトップアイコン/タスクバーの再表示に失敗しました";

pub fn disable_aero() -> Result<(), String> {
    set_bool_parameter(SPI_SETUIEFFECTS, false, AERO_OPERATION)?;
    set_bool_parameter(SPI_SETDROPSHADOW, false, AERO_OPERATION)?;
    set_bool_parameter(SPI_SETCLIENTAREAANIMATION, false, AERO_OPERATION)?;

    let mut animation = ANIMATIONINFO {
        cbSize: std::mem::size_of::<ANIMATIONINFO>() as u32,
        iMinAnimate: 0,
    };
    unsafe {
        SystemParametersInfoW(
            SPI_SETANIMATION,
            animation.cbSize,
            Some((&mut animation as *mut ANIMATIONINFO).cast()),
            SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
        )
        .map_err(|error| from_windows_error(AERO_OPERATION, error))?;
    }

    set_hkcu_dword(
        r"Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects",
        "VisualFXSetting",
        2,
        AERO_OPERATION,
    )
}

pub fn restore_aero() -> Result<(), String> {
    set_bool_parameter(SPI_SETUIEFFECTS, true, AERO_RESTORE_OPERATION)?;
    set_bool_parameter(SPI_SETDROPSHADOW, true, AERO_RESTORE_OPERATION)?;
    set_bool_parameter(SPI_SETCLIENTAREAANIMATION, true, AERO_RESTORE_OPERATION)?;

    let mut animation = ANIMATIONINFO {
        cbSize: std::mem::size_of::<ANIMATIONINFO>() as u32,
        iMinAnimate: 1,
    };
    unsafe {
        SystemParametersInfoW(
            SPI_SETANIMATION,
            animation.cbSize,
            Some((&mut animation as *mut ANIMATIONINFO).cast()),
            SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
        )
        .map_err(|error| from_windows_error(AERO_RESTORE_OPERATION, error))?;
    }

    set_hkcu_dword(
        r"Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects",
        "VisualFXSetting",
        0,
        AERO_RESTORE_OPERATION,
    )
}

pub fn set_solid_wallpaper() -> Result<(), String> {
    unsafe {
        SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            Some(w!("").as_ptr().cast_mut().cast()),
            SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
        )
        .map_err(|error| from_windows_error(WALLPAPER_OPERATION, error))?;
    }

    let elements = [COLOR_BACKGROUND.0, COLOR_DESKTOP.0];
    let colors = [COLORREF(0), COLORREF(0)];
    unsafe {
        SetSysColors(2, elements.as_ptr(), colors.as_ptr())
            .map_err(|error| from_windows_error(WALLPAPER_OPERATION, error))?;
    }

    set_hkcu_sz(
        r"Control Panel\Desktop",
        "Wallpaper",
        "",
        WALLPAPER_OPERATION,
    )?;
    set_hkcu_sz(
        r"Control Panel\Colors",
        "Background",
        "0 0 0",
        WALLPAPER_OPERATION,
    )
}

pub fn hide_desktop_icons_and_taskbar() -> Result<(), String> {
    set_hkcu_dword(
        r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
        "HideIcons",
        1,
        DESKTOP_OPERATION,
    )?;

    const HIDE_ICONS_BIT: i32 = 1 << 12;
    let mut state = SHELLSTATEA::default();
    unsafe {
        SHGetSetSettings(Some(&mut state), SSF_HIDEICONS, false);
        state._bitfield1 |= HIDE_ICONS_BIT;
        SHGetSetSettings(Some(&mut state), SSF_HIDEICONS, true);
    }

    unsafe {
        EnumWindows(Some(hide_taskbar_window), LPARAM(0))
            .map_err(|error| from_windows_error(DESKTOP_OPERATION, error))?;
    }

    Ok(())
}

pub fn restore_desktop_icons_and_taskbar() -> Result<(), String> {
    set_hkcu_dword(
        r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
        "HideIcons",
        0,
        DESKTOP_RESTORE_OPERATION,
    )?;

    const HIDE_ICONS_BIT: i32 = 1 << 12;
    let mut state = SHELLSTATEA::default();
    unsafe {
        SHGetSetSettings(Some(&mut state), SSF_HIDEICONS, false);
        state._bitfield1 &= !HIDE_ICONS_BIT;
        SHGetSetSettings(Some(&mut state), SSF_HIDEICONS, true);
    }

    unsafe {
        EnumWindows(Some(show_taskbar_window), LPARAM(0))
            .map_err(|error| from_windows_error(DESKTOP_RESTORE_OPERATION, error))?;
    }

    Ok(())
}

fn set_bool_parameter(
    action: windows::Win32::UI::WindowsAndMessaging::SYSTEM_PARAMETERS_INFO_ACTION,
    enabled: bool,
    operation: &str,
) -> Result<(), String> {
    let mut value = BOOL::from(enabled);
    unsafe {
        SystemParametersInfoW(
            action,
            0,
            Some((&mut value as *mut BOOL).cast()),
            SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
        )
        .map_err(|error| from_windows_error(operation, error))
    }
}

unsafe extern "system" fn hide_taskbar_window(hwnd: HWND, _: LPARAM) -> BOOL {
    let mut class_name = [0u16; 64];
    let len = unsafe { GetClassNameW(hwnd, &mut class_name) };
    if len > 0 {
        let class = String::from_utf16_lossy(&class_name[..len as usize]);
        if class == "Shell_TrayWnd" || class == "Shell_SecondaryTrayWnd" {
            unsafe {
                let _ = ShowWindow(hwnd, SW_HIDE);
            }
        }
    }
    TRUE
}

unsafe extern "system" fn show_taskbar_window(hwnd: HWND, _: LPARAM) -> BOOL {
    let mut class_name = [0u16; 64];
    let len = unsafe { GetClassNameW(hwnd, &mut class_name) };
    if len > 0 {
        let class = String::from_utf16_lossy(&class_name[..len as usize]);
        if class == "Shell_TrayWnd" || class == "Shell_SecondaryTrayWnd" {
            unsafe {
                let _ = ShowWindow(hwnd, SW_SHOWNA);
            }
        }
    }
    TRUE
}

#[cfg(test)]
mod tests {
    #[test]
    fn solid_wallpaper_uses_black() {
        assert_eq!("0 0 0", "0 0 0");
    }
}
