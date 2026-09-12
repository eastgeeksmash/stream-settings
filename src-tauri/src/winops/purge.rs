use super::folders::{local_app_data, remove_tree_if_exists, roaming_app_data};
use super::process::terminate_process_tree_by_name;

const OBS_OPERATION: &str = "OBS設定の削除に失敗しました";
const VMIX_OPERATION: &str = "vMix設定の削除に失敗しました";

pub fn purge_obs_settings() -> Result<(), String> {
    let _ = terminate_process_tree_by_name("obs64.exe", OBS_OPERATION);
    let _ = terminate_process_tree_by_name("obs32.exe", OBS_OPERATION);
    let roaming = roaming_app_data(OBS_OPERATION)?.join("obs-studio");
    remove_tree_if_exists(&roaming, OBS_OPERATION)
}

pub fn purge_vmix_settings() -> Result<(), String> {
    let _ = terminate_process_tree_by_name("vMix.exe", VMIX_OPERATION);
    let local = local_app_data(VMIX_OPERATION)?;
    remove_tree_if_exists(
        &local.join("StudioCoast Pty Ltd").join("vMix"),
        VMIX_OPERATION,
    )?;
    remove_tree_if_exists(&local.join("StudioCoast_Pty_Ltd"), VMIX_OPERATION)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    fn vmix_paths_match_issue_description() {
        let local = PathBuf::from(r"C:\Users\SPDG\AppData\Local");
        assert_eq!(
            local.join("StudioCoast Pty Ltd").join("vMix"),
            PathBuf::from(r"C:\Users\SPDG\AppData\Local\StudioCoast Pty Ltd\vMix")
        );
        assert_eq!(
            local.join("StudioCoast_Pty_Ltd"),
            PathBuf::from(r"C:\Users\SPDG\AppData\Local\StudioCoast_Pty_Ltd")
        );
    }
}
