use windows::Win32::Foundation::{LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    SendMessageTimeoutW, HWND_BROADCAST, SMTO_ABORTIFHUNG, WM_SETTINGCHANGE,
};

use super::registry::{
    enum_hkcu_subkeys, get_hkcu_default_sz, set_hkcu_default_sz, set_hkcu_dword,
};

const DUCKING_OPERATION: &str = "通話時の音量下げ無効化に失敗しました";
const SCHEME_OPERATION: &str = "Windowsサウンドの変更に失敗しました";
const SCHEMES_KEY: &str = r"AppEvents\Schemes";
const APPS_KEY: &str = r"AppEvents\Schemes\Apps";

pub const DUCKING_DO_NOTHING: u32 = 3;
pub const NO_SOUNDS_SCHEME: &str = ".None";

pub fn disable_call_ducking() -> Result<(), String> {
    set_hkcu_dword(
        r"Software\Microsoft\Multimedia\Audio",
        "UserDuckingPreference",
        DUCKING_DO_NOTHING,
        DUCKING_OPERATION,
    )
}

pub fn set_no_sounds_scheme() -> Result<(), String> {
    set_hkcu_default_sz(SCHEMES_KEY, NO_SOUNDS_SCHEME, SCHEME_OPERATION)?;
    apply_none_to_current(APPS_KEY, SCHEME_OPERATION)?;
    notify_sound_scheme_changed();
    Ok(())
}

fn apply_none_to_current(subkey: &str, operation: &str) -> Result<(), String> {
    let children = enum_hkcu_subkeys(subkey, operation)?;
    if is_sound_event(&children) {
        let none_key = format!(r"{subkey}\.None");
        let current_key = format!(r"{subkey}\.Current");
        let value = get_hkcu_default_sz(&none_key, operation)?.unwrap_or_default();
        set_hkcu_default_sz(&current_key, &value, operation)?;
        return Ok(());
    }

    for child in children {
        apply_none_to_current(&format!(r"{subkey}\{child}"), operation)?;
    }
    Ok(())
}

fn is_sound_event(children: &[String]) -> bool {
    children.iter().any(|name| name == ".Current")
}

fn notify_sound_scheme_changed() {
    let parameter: Vec<u16> = "AppEvents".encode_utf16().chain(std::iter::once(0)).collect();
    unsafe {
        let _ = SendMessageTimeoutW(
            HWND_BROADCAST,
            WM_SETTINGCHANGE,
            WPARAM(0),
            LPARAM(parameter.as_ptr() as isize),
            SMTO_ABORTIFHUNG,
            1000,
            None,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ducking_preference_does_nothing() {
        assert_eq!(DUCKING_DO_NOTHING, 3);
    }

    #[test]
    fn no_sounds_scheme_name() {
        assert_eq!(NO_SOUNDS_SCHEME, ".None");
    }

    #[test]
    fn event_keys_stop_before_scheme_slots() {
        assert!(is_sound_event(&[
            ".Current".to_string(),
            ".Default".to_string(),
            ".None".to_string(),
        ]));
        assert!(!is_sound_event(&[".Default".to_string(), "Explorer".to_string()]));
    }
}
