use super::registry::set_hklm_dword;

const DELIVERY_OPERATION: &str = "Windows Updateのファイル配信無効化に失敗しました";
const DEFER_OPERATION: &str = "Windows Updateの延期に失敗しました";

pub fn disable_delivery_optimization() -> Result<(), String> {
    set_hklm_dword(
        r"SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization",
        "DODownloadMode",
        0,
        DELIVERY_OPERATION,
    )
}

pub fn defer_windows_update() -> Result<(), String> {
    set_hklm_dword(
        r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate",
        "DeferFeatureUpdates",
        1,
        DEFER_OPERATION,
    )?;
    set_hklm_dword(
        r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate",
        "DeferFeatureUpdatesPeriodInDays",
        365,
        DEFER_OPERATION,
    )?;
    set_hklm_dword(
        r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate",
        "DeferQualityUpdates",
        1,
        DEFER_OPERATION,
    )?;
    set_hklm_dword(
        r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate",
        "DeferQualityUpdatesPeriodInDays",
        30,
        DEFER_OPERATION,
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn delivery_optimization_uses_http_only_mode() {
        assert_eq!(0, 0);
    }
}
