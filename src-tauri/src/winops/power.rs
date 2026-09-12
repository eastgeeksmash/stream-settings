use windows::core::GUID;
use windows::Win32::Foundation::{ERROR_MORE_DATA, ERROR_SUCCESS};
use windows::Win32::System::Power::{
    PowerReadFriendlyName, PowerSetActiveScheme, PowerWriteACValueIndex,
};

use super::error::from_win32_error;

pub const HIGH_PERFORMANCE_SCHEME: GUID = GUID::from_u128(0x8c5e7fda_e8bf_4a96_9a85_a6e23a8c635c);
pub const SLEEP_SUBGROUP: GUID = GUID::from_u128(0x238c9fa8_0aad_41ed_83f4_97be242c8f20);
pub const STANDBY_TIMEOUT: GUID = GUID::from_u128(0x29f6c1db_86da_48c5_9fdb_f2b67b1f44da);
pub const SYSTEM_BUTTON_SUBGROUP: GUID = GUID::from_u128(0x4f971e89_eebd_4455_a8de_9e59040e7347);
pub const POWERBUTTON_ACTION: GUID = GUID::from_u128(0x7648efa3_dd9c_4e3e_b566_50f929386280);

const SCHEME_OPERATION: &str = "電源プランの変更に失敗しました";
const SLEEP_OPERATION: &str = "スリープ設定の変更に失敗しました";
const BUTTON_OPERATION: &str = "電源ボタン設定の変更に失敗しました";
const APPLY_OPERATION: &str = "電源設定の適用に失敗しました";

pub fn change_power_settings() -> Result<(), String> {
    ensure_high_performance_scheme_exists()?;

    let status = unsafe { PowerSetActiveScheme(None, Some(&HIGH_PERFORMANCE_SCHEME)) };
    if status != ERROR_SUCCESS {
        return Err(from_win32_error(SCHEME_OPERATION, status));
    }

    write_ac_value(&SLEEP_SUBGROUP, &STANDBY_TIMEOUT, 0, SLEEP_OPERATION)?;
    write_ac_value(
        &SYSTEM_BUTTON_SUBGROUP,
        &POWERBUTTON_ACTION,
        0,
        BUTTON_OPERATION,
    )?;

    let status = unsafe { PowerSetActiveScheme(None, Some(&HIGH_PERFORMANCE_SCHEME)) };
    if status != ERROR_SUCCESS {
        return Err(from_win32_error(APPLY_OPERATION, status));
    }

    Ok(())
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

fn write_ac_value(
    subgroup: &GUID,
    setting: &GUID,
    value: u32,
    operation: &str,
) -> Result<(), String> {
    let status = unsafe {
        PowerWriteACValueIndex(
            None,
            &HIGH_PERFORMANCE_SCHEME,
            Some(subgroup),
            Some(setting),
            value,
        )
    };

    if status != ERROR_SUCCESS {
        return Err(from_win32_error(operation, status));
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
    }
}
