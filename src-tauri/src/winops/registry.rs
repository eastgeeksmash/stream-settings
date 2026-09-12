use windows::core::{w, PWSTR};
use windows::Win32::Foundation::{ERROR_FILE_NOT_FOUND, ERROR_SUCCESS};
use windows::Win32::System::Registry::{
    RegCloseKey, RegCreateKeyExW, RegDeleteValueW, RegEnumKeyExW, RegOpenKeyExW, RegQueryValueExW,
    RegSetValueExW, HKEY, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_READ, KEY_WRITE, REG_DWORD,
    REG_OPTION_NON_VOLATILE, REG_SZ, REG_VALUE_TYPE,
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

pub fn delete_hklm_value(subkey: &str, name: &str, operation: &str) -> Result<(), String> {
    let subkey: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();
    let name: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
    let mut key = HKEY::default();

    let status = unsafe {
        RegCreateKeyExW(
            HKEY_LOCAL_MACHINE,
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

    let status = unsafe { RegDeleteValueW(key, windows::core::PCWSTR(name.as_ptr())) };
    unsafe {
        let _ = RegCloseKey(key);
    }

    if status != ERROR_SUCCESS && status != ERROR_FILE_NOT_FOUND {
        return Err(from_win32_error(operation, status));
    }

    Ok(())
}

pub fn enum_hklm_subkeys(subkey: &str, operation: &str) -> Result<Vec<String>, String> {
    let key = open_hklm(subkey, KEY_READ, operation)?;
    let mut names = Vec::new();
    for index in 0..512 {
        let mut name = [0u16; 256];
        let mut name_len = name.len() as u32;
        let status = unsafe {
            RegEnumKeyExW(
                key,
                index,
                Some(PWSTR(name.as_mut_ptr())),
                &mut name_len,
                None,
                None,
                None,
                None,
            )
        };
        if status != ERROR_SUCCESS {
            break;
        }
        if let Ok(value) = String::from_utf16(&name[..name_len as usize]) {
            if !value.is_empty() {
                names.push(value);
            }
        }
    }
    unsafe {
        let _ = RegCloseKey(key);
    }
    Ok(names)
}

pub fn get_hklm_sz(subkey: &str, name: &str, operation: &str) -> Result<Option<String>, String> {
    let key = match open_hklm(subkey, KEY_READ, operation) {
        Ok(key) => key,
        Err(_) => return Ok(None),
    };
    let result = query_sz(key, name);
    unsafe {
        let _ = RegCloseKey(key);
    }
    result
}

pub fn set_existing_hklm_number(
    subkey: &str,
    name: &str,
    value: u32,
    operation: &str,
) -> Result<bool, String> {
    let key = match open_hklm(subkey, KEY_READ | KEY_WRITE, operation) {
        Ok(key) => key,
        Err(_) => return Ok(false),
    };
    let result = set_existing_number(key, name, value, operation);
    unsafe {
        let _ = RegCloseKey(key);
    }
    result
}

fn open_hklm(
    subkey: &str,
    access: windows::Win32::System::Registry::REG_SAM_FLAGS,
    operation: &str,
) -> Result<HKEY, String> {
    let subkey: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();
    let mut key = HKEY::default();
    let status = unsafe {
        RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            windows::core::PCWSTR(subkey.as_ptr()),
            Some(0),
            access,
            &mut key,
        )
    };
    if status != ERROR_SUCCESS {
        return Err(from_win32_error(operation, status));
    }
    Ok(key)
}

fn query_sz(key: HKEY, name: &str) -> Result<Option<String>, String> {
    let name: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
    let mut value_type = REG_VALUE_TYPE::default();
    let mut size = 0u32;
    let status = unsafe {
        RegQueryValueExW(
            key,
            windows::core::PCWSTR(name.as_ptr()),
            None,
            Some(&mut value_type),
            None,
            Some(&mut size),
        )
    };
    if status == ERROR_FILE_NOT_FOUND || size == 0 {
        return Ok(None);
    }
    if status != ERROR_SUCCESS {
        return Ok(None);
    }
    let mut buffer = vec![0u8; size as usize];
    let status = unsafe {
        RegQueryValueExW(
            key,
            windows::core::PCWSTR(name.as_ptr()),
            None,
            Some(&mut value_type),
            Some(buffer.as_mut_ptr()),
            Some(&mut size),
        )
    };
    if status != ERROR_SUCCESS || value_type != REG_SZ {
        return Ok(None);
    }
    let utf16: Vec<u16> = buffer
        .chunks_exact(2)
        .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
        .take_while(|unit| *unit != 0)
        .collect();
    Ok(String::from_utf16(&utf16).ok())
}

fn query_type(key: HKEY, name: &str) -> Option<REG_VALUE_TYPE> {
    let name: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
    let mut value_type = REG_VALUE_TYPE::default();
    let mut size = 0u32;
    let status = unsafe {
        RegQueryValueExW(
            key,
            windows::core::PCWSTR(name.as_ptr()),
            None,
            Some(&mut value_type),
            None,
            Some(&mut size),
        )
    };
    if status == ERROR_SUCCESS {
        Some(value_type)
    } else {
        None
    }
}

fn set_existing_number(
    key: HKEY,
    name: &str,
    value: u32,
    operation: &str,
) -> Result<bool, String> {
    let Some(value_type) = query_type(key, name) else {
        return Ok(false);
    };
    let name_wide: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
    let status = if value_type == REG_DWORD {
        unsafe {
            RegSetValueExW(
                key,
                windows::core::PCWSTR(name_wide.as_ptr()),
                None,
                REG_DWORD,
                Some(&value.to_le_bytes()),
            )
        }
    } else if value_type == REG_SZ {
        let data: Vec<u8> = value
            .to_string()
            .encode_utf16()
            .chain(std::iter::once(0))
            .flat_map(u16::to_le_bytes)
            .collect();
        unsafe {
            RegSetValueExW(
                key,
                windows::core::PCWSTR(name_wide.as_ptr()),
                None,
                REG_SZ,
                Some(&data),
            )
        }
    } else {
        return Ok(false);
    };
    if status != ERROR_SUCCESS {
        return Err(from_win32_error(operation, status));
    }
    Ok(true)
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
