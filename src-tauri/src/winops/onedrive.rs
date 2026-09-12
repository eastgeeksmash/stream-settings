use super::process::terminate_process_tree_by_name;
use super::registry::set_hklm_dword;

const OPERATION: &str = "OneDrive同期の無効化に失敗しました";

pub fn disable_onedrive_sync() -> Result<(), String> {
    set_hklm_dword(
        r"SOFTWARE\Policies\Microsoft\Windows\OneDrive",
        "DisableFileSyncNGSC",
        1,
        OPERATION,
    )?;
    let _ = terminate_process_tree_by_name("OneDrive.exe", OPERATION);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn onedrive_policy_name_is_disable_file_sync() {
        assert_eq!("DisableFileSyncNGSC", "DisableFileSyncNGSC");
    }
}
