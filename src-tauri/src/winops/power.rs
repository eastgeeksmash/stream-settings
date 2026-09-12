use windows::core::GUID;
use windows::Win32::Foundation::{
    ERROR_FILE_NOT_FOUND, ERROR_MORE_DATA, ERROR_NOT_FOUND, ERROR_SUCCESS, WIN32_ERROR,
};
use windows::Win32::System::Power::{
    PowerReadFriendlyName, PowerSetActiveScheme, PowerWriteACValueIndex, PowerWriteDCValueIndex,
};

use super::error::from_win32_error;

pub const HIGH_PERFORMANCE_SCHEME: GUID = GUID::from_u128(0x8c5e7fda_e8bf_4a96_9a85_a6e23a8c635c);
pub const SLEEP_SUBGROUP: GUID = GUID::from_u128(0x238c9fa8_0aad_41ed_83f4_97be242c8f20);
pub const STANDBY_TIMEOUT: GUID = GUID::from_u128(0x29f6c1db_86da_48c5_9fdb_f2b67b1f44da);
pub const SYSTEM_BUTTON_SUBGROUP: GUID = GUID::from_u128(0x4f971e89_eebd_4455_a8de_9e59040e7347);
pub const POWERBUTTON_ACTION: GUID = GUID::from_u128(0x7648efa3_dd9c_4e3e_b566_50f929386280);
pub const VIDEO_SUBGROUP: GUID = GUID::from_u128(0x7516b95f_f776_4464_8c53_06167f40cc99);
pub const VIDEO_POWERDOWN_TIMEOUT: GUID = GUID::from_u128(0x3c0bc019_c8a8_4e32_8ff6_4ea39ae5c035);
pub const HIBERNATE_TIMEOUT: GUID = GUID::from_u128(0x9d7815a6_7ee4_458c_8c55_4ec09ba02559);
pub const PCIEXPRESS_SUBGROUP: GUID = GUID::from_u128(0x501a4ea5_b76e_4869_973a_07ad1620bb16);
pub const PCIEXPRESS_ASPM: GUID = GUID::from_u128(0xee12f906_d277_404b_b6da_e5fa1a576df5);
pub const USB_SUBGROUP: GUID = GUID::from_u128(0x2a737441_1930_4402_8d77_b2bebba308a3);
pub const USB_SELECTIVE_SUSPEND: GUID = GUID::from_u128(0x48e6b7a6_50f5_4782_a5d4_143dd0d2b298);

const SCHEME_OPERATION: &str = "電源プランの変更に失敗しました";
const SLEEP_OPERATION: &str = "スリープ設定の変更に失敗しました";
const HIBERNATE_OPERATION: &str = "休止設定の変更に失敗しました";
const DISPLAY_OPERATION: &str = "自動電源オフ設定の変更に失敗しました";
const BUTTON_OPERATION: &str = "電源ボタン設定の変更に失敗しました";
const APPLY_OPERATION: &str = "電源設定の適用に失敗しました";

pub fn change_power_settings() -> Result<(), String> {
    ensure_high_performance_scheme_exists()?;

    let status = unsafe { PowerSetActiveScheme(None, Some(&HIGH_PERFORMANCE_SCHEME)) };
    if status != ERROR_SUCCESS {
        return Err(from_win32_error(SCHEME_OPERATION, status));
    }

    write_value(&SLEEP_SUBGROUP, &STANDBY_TIMEOUT, 0, SLEEP_OPERATION, true)?;
    write_value(
        &SLEEP_SUBGROUP,
        &HIBERNATE_TIMEOUT,
        0,
        HIBERNATE_OPERATION,
        false,
    )?;
    write_value(
        &VIDEO_SUBGROUP,
        &VIDEO_POWERDOWN_TIMEOUT,
        0,
        DISPLAY_OPERATION,
        false,
    )?;
    write_value(
        &SYSTEM_BUTTON_SUBGROUP,
        &POWERBUTTON_ACTION,
        0,
        BUTTON_OPERATION,
        false,
    )?;

    let status = unsafe { PowerSetActiveScheme(None, Some(&HIGH_PERFORMANCE_SCHEME)) };
    if status != ERROR_SUCCESS {
        return Err(from_win32_error(APPLY_OPERATION, status));
    }

    Ok(())
}

pub fn disable_bus_power_saving() -> Result<(), String> {
    write_value(
        &PCIEXPRESS_SUBGROUP,
        &PCIEXPRESS_ASPM,
        0,
        "バスの省電力設定の変更に失敗しました",
        false,
    )?;
    write_value(
        &USB_SUBGROUP,
        &USB_SELECTIVE_SUSPEND,
        0,
        "USBの省電力設定の変更に失敗しました",
        false,
    )
}

fn ensure_high_performance_scheme_exists() -> Result<(), String> {
    let mut size = 0u32;
    let status = unsafe {
        PowerReadFriendlyName(
            None,
            Some(&HIGH_PERFORMANCE_SCHEME),
            None,
            None,
            None,
            &mut size,
        )
    };

    if status == ERROR_SUCCESS || status == ERROR_MORE_DATA {
        return Ok(());
    }

    Err(from_win32_error(
        "高パフォーマンス電源プランが見つかりません",
        status,
    ))
}

fn is_missing_setting(status: WIN32_ERROR) -> bool {
    status == ERROR_FILE_NOT_FOUND || status == ERROR_NOT_FOUND
}

fn write_value(
    subgroup: &GUID,
    setting: &GUID,
    value: u32,
    operation: &str,
    required: bool,
) -> Result<(), String> {
    let ac = unsafe {
        PowerWriteACValueIndex(
            None,
            &HIGH_PERFORMANCE_SCHEME,
            Some(subgroup),
            Some(setting),
            value,
        )
    };
    let dc = WIN32_ERROR(unsafe {
        PowerWriteDCValueIndex(
            None,
            &HIGH_PERFORMANCE_SCHEME,
            Some(subgroup),
            Some(setting),
            value,
        )
    });

    for status in [ac, dc] {
        if status == ERROR_SUCCESS || is_missing_setting(status) {
            continue;
        }
        return Err(from_win32_error(operation, status));
    }

    if required && ac != ERROR_SUCCESS {
        return Err(from_win32_error(operation, ac));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn guid_from_hyphenated(value: &str) -> GUID {
        let hex: String = value.chars().filter(|c| *c != '-').collect();
        let parsed = u128::from_str_radix(&hex, 16).expect("valid GUID hex");
        GUID::from_u128(parsed)
    }

    #[test]
    fn power_guids_match_previous_powershell_implementation() {
        assert_eq!(
            HIGH_PERFORMANCE_SCHEME,
            guid_from_hyphenated("8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c")
        );
        assert_eq!(
            SYSTEM_BUTTON_SUBGROUP,
            guid_from_hyphenated("4f971e89-eebd-4455-a8de-9e59040e7347")
        );
        assert_eq!(
            POWERBUTTON_ACTION,
            guid_from_hyphenated("7648efa3-dd9c-4e3e-b566-50f929386280")
        );
        assert_eq!(
            SLEEP_SUBGROUP,
            guid_from_hyphenated("238c9fa8-0aad-41ed-83f4-97be242c8f20")
        );
        assert_eq!(
            STANDBY_TIMEOUT,
            guid_from_hyphenated("29f6c1db-86da-48c5-9fdb-f2b67b1f44da")
        );
        assert_eq!(
            VIDEO_POWERDOWN_TIMEOUT,
            guid_from_hyphenated("3c0bc019-c8a8-4e32-8ff6-4ea39ae5c035")
        );
        assert_eq!(
            HIBERNATE_TIMEOUT,
            guid_from_hyphenated("9d7815a6-7ee4-458c-8c55-4ec09ba02559")
        );
        assert_eq!(
            PCIEXPRESS_ASPM,
            guid_from_hyphenated("ee12f906-d277-404b-b6da-e5fa1a576df5")
        );
        assert_eq!(
            USB_SELECTIVE_SUSPEND,
            guid_from_hyphenated("48e6b7a6-50f5-4782-a5d4-143dd0d2b298")
        );
    }

    #[test]
    fn missing_power_setting_is_skippable() {
        assert!(is_missing_setting(ERROR_FILE_NOT_FOUND));
        assert!(is_missing_setting(ERROR_NOT_FOUND));
        assert!(!is_missing_setting(ERROR_SUCCESS));
    }
}
