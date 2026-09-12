use windows::core::Error;

const WIN32_FACILITY_MASK: u32 = 0xFFFF_0000;
const WIN32_FACILITY: u32 = 0x8007_0000;
const ERROR_FILE_NOT_FOUND: u32 = 2;
const ERROR_ACCESS_DENIED: u32 = 5;
const ERROR_CONNECTION_UNAVAIL: u32 = 1201;
const ERROR_NO_NETWORK: u32 = 1222;
const ERROR_PRIVILEGE_NOT_HELD: u32 = 1314;
const ERROR_NOT_FOUND: u32 = 1168;
const ERROR_NOT_CONNECTED: u32 = 2250;
const CLASS_NOT_REGISTERED: u32 = 0x8004_0154;
const CLASS_NOT_REGISTERED_CODE: u32 = 0x0154;

pub fn win32_code_from_hresult(hr: i32) -> u32 {
    let value = hr as u32;
    if value & WIN32_FACILITY_MASK == WIN32_FACILITY {
        value & 0xFFFF
    } else {
        value
    }
}

pub fn detail_for_win32(code: u32) -> String {
    match code {
        ERROR_ACCESS_DENIED | ERROR_PRIVILEGE_NOT_HELD => {
            "権限が不足しています。管理者として実行してください。".to_string()
        }
        ERROR_FILE_NOT_FOUND | ERROR_NOT_FOUND => "対象が見つかりません。".to_string(),
        other => format!("エラーコード {other}"),
    }
}

pub fn is_skippable_network_error(error: &Error) -> bool {
    let code = win32_code_from_hresult(error.code().0);
    matches!(
        code,
        ERROR_FILE_NOT_FOUND
            | ERROR_NOT_FOUND
            | ERROR_CONNECTION_UNAVAIL
            | ERROR_NO_NETWORK
            | ERROR_NOT_CONNECTED
    ) || error.message().contains("does not exist")
}

pub fn is_class_not_registered(status: u32) -> bool {
    status == CLASS_NOT_REGISTERED || status == CLASS_NOT_REGISTERED_CODE
}

pub fn format_operation_error(prefix: &str, code: u32) -> String {
    format!("{prefix}: {}", detail_for_win32(code))
}

pub fn from_windows_error(prefix: &str, error: Error) -> String {
    let hr = error.code().0;
    let code = win32_code_from_hresult(hr);
    match code {
        ERROR_ACCESS_DENIED | ERROR_PRIVILEGE_NOT_HELD | ERROR_FILE_NOT_FOUND | ERROR_NOT_FOUND => {
            format_operation_error(prefix, code)
        }
        _ => {
            let message = error.message();
            if message.is_empty() {
                format!("{prefix}: {}", detail_for_win32(code))
            } else {
                format!("{prefix}: {message}")
            }
        }
    }
}

pub fn from_win32_error(prefix: &str, error: windows::Win32::Foundation::WIN32_ERROR) -> String {
    from_windows_error(prefix, Error::from(error))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_access_denied_hresult_to_win32_code() {
        assert_eq!(win32_code_from_hresult(0x8007_0005u32 as i32), 5);
    }

    #[test]
    fn maps_privilege_errors_to_elevation_message() {
        let message = format_operation_error("ネットワーク設定の変更に失敗しました", 5);
        assert!(message.contains("管理者として実行"));
        assert_eq!(
            detail_for_win32(1314),
            "権限が不足しています。管理者として実行してください。"
        );
    }

    #[test]
    fn maps_not_found_errors() {
        assert_eq!(detail_for_win32(2), "対象が見つかりません。");
        assert_eq!(detail_for_win32(1168), "対象が見つかりません。");
    }

    #[test]
    fn skips_stale_network_connection_errors() {
        let error = Error::from(windows::Win32::Foundation::WIN32_ERROR(ERROR_NOT_CONNECTED));
        assert!(is_skippable_network_error(&error));
        assert!(is_class_not_registered(0x8004_0154));
    }
}
