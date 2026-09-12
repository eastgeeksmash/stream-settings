use windows::core::w;
use windows::Win32::Foundation::ERROR_SUCCESS;
use windows::Win32::System::Registry::{
    RegCloseKey, RegCreateKeyExW, RegSetValueExW, HKEY, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE,
    KEY_WRITE, REG_DWORD, REG_OPTION_NON_VOLATILE, REG_SZ,
};

use super::error::from_win32_error;

const NOTIFICATION_OPERATION: &str = "通知センターの無効化に失敗しました";

pub struct RegistryDwordValue {
    pub subkey: &'static str,
    pub name: &'static str,
    pub value: u32,
}

pub const NOTIFICATION_VALUES: &[RegistryDwordValue] = &[
    RegistryDwordValue {
        subkey: r"Software\Policies\Microsoft\Windows\Explorer",
        name: "DisableNotificationCenter",
        value: 1,
    },
    RegistryDwordValue {
        subkey: r"Software\Microsoft\Windows\CurrentVersion\PushNotifications",
        name: "ToastEnabled",
        value: 0,
    },
];

pub const RESTART_HINT: &str =
    "設定の反映にはエクスプローラーの再起動、サインアウト、または再起動が必要な場合があります。";

pub fn disable_windows_notifications() -> Result<(), String> {
    for value in NOTIFICATION_VALUES {
        set_hkcu_dword(
            value.subkey,
            value.name,
            value.value,
            NOTIFICATION_OPERATION,
        )?;
    }
    let _ = RESTART_HINT;
    Ok(())
}

pub fn set_hkcu_dword(subkey: &str, name: &str, value: u32, operation: &str) -> Result<(), String> {
    set_dword(HKEY_CURRENT_USER, subkey, name, value, operation)
}

pub fn set_hklm_dword(subkey: &str, name: &str, value: u32, operation: &str) -> Result<(), String> {
    set_dword(HKEY_LOCAL_MACHINE, subkey, name, value, operation)
}

pub fn set_hkcu_sz(subkey: &str, name: &str, value: &str, operation: &str) -> Result<(), String> {
    let data: Vec<u8> = value
        .encode_utf16()
        .chain(std::iter::once(0))
        .flat_map(u16::to_le_bytes)
        .collect();
    set_value(HKEY_CURRENT_USER, subkey, name, REG_SZ, &data, operation)
}

fn set_dword(
    root: HKEY,
    subkey: &str,
    name: &str,
    value: u32,
    operation: &str,
) -> Result<(), String> {
    set_value(
        root,
        subkey,
        name,
        REG_DWORD,
        &value.to_le_bytes(),
        operation,
    )
}

fn set_value(
    root: HKEY,
    subkey: &str,
    name: &str,
    value_type: windows::Win32::System::Registry::REG_VALUE_TYPE,
    data: &[u8],
    operation: &str,
) -> Result<(), String> {
    let subkey: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();
    let name: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
    let mut key = HKEY::default();

    let status = unsafe {
        RegCreateKeyExW(
            root,
            windows::core::PCWSTR(subkey.as_ptr()),
            None,
            w!(""),
            REG_OPTION_NON_VOLATILE,
            KEY_WRITE,
            None,
            &mut key,
            None,
        )
    };
    if status != ERROR_SUCCESS {
        return Err(from_win32_error(operation, status));
    }

    let status = unsafe {
        RegSetValueExW(
            key,
            windows::core::PCWSTR(name.as_ptr()),
            None,
            value_type,
            Some(data),
        )
    };
    unsafe {
        let _ = RegCloseKey(key);
    }

    if status != ERROR_SUCCESS {
        return Err(from_win32_error(operation, status));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notification_registry_values_match_previous_implementation() {
        assert_eq!(NOTIFICATION_VALUES.len(), 2);
        assert_eq!(
            NOTIFICATION_VALUES[0].subkey,
            r"Software\Policies\Microsoft\Windows\Explorer"
        );
        assert_eq!(NOTIFICATION_VALUES[0].name, "DisableNotificationCenter");
        assert_eq!(NOTIFICATION_VALUES[0].value, 1);
        assert_eq!(
            NOTIFICATION_VALUES[1].subkey,
            r"Software\Microsoft\Windows\CurrentVersion\PushNotifications"
        );
        assert_eq!(NOTIFICATION_VALUES[1].name, "ToastEnabled");
        assert_eq!(NOTIFICATION_VALUES[1].value, 0);
    }

    #[test]
    fn restart_hint_is_available_without_ui_change() {
        assert!(RESTART_HINT.contains("再起動"));
    }
}
