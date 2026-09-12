use std::path::Path;

use super::folders::{
    local_app_data, program_data, remove_file_if_exists, remove_tree_if_exists, roaming_app_data,
};
use super::process::terminate_process_tree_by_name;

const OBS_OPERATION: &str = "OBS設定の削除に失敗しました";
const VMIX_OPERATION: &str = "vMix設定の削除に失敗しました";
const VMIX_REGISTRATION_OPERATION: &str = "vMix登録キーの削除に失敗しました";
const VMIX_LICENSE_FILES: &[&str] = &["es.xml", "s.xml"];

pub fn purge_obs_settings() -> Result<(), String> {
    let _ = terminate_process_tree_by_name("obs64.exe", OBS_OPERATION);
    let _ = terminate_process_tree_by_name("obs32.exe", OBS_OPERATION);
    let roaming = roaming_app_data(OBS_OPERATION)?.join("obs-studio");
    remove_tree_if_exists(&roaming, OBS_OPERATION)
}

pub fn purge_vmix_settings() -> Result<(), String> {
    stop_vmix(VMIX_OPERATION);
    let local = local_app_data(VMIX_OPERATION)?;
    remove_tree_if_exists(
        &local.join("StudioCoast Pty Ltd").join("vMix"),
        VMIX_OPERATION,
    )?;
    remove_tree_if_exists(&local.join("StudioCoast_Pty_Ltd"), VMIX_OPERATION)
}

pub fn reset_vmix_registration() -> Result<(), String> {
    stop_vmix(VMIX_REGISTRATION_OPERATION);
    let vmix = program_data(VMIX_REGISTRATION_OPERATION)?.join("vMix");
    for name in VMIX_LICENSE_FILES {
        remove_file_if_exists(&vmix.join(name), VMIX_REGISTRATION_OPERATION)?;
    }
    remove_license_backups(&vmix.join("backups"), VMIX_REGISTRATION_OPERATION)
}

fn stop_vmix(operation: &str) {
    let _ = terminate_process_tree_by_name("vMix.exe", operation);
    let _ = terminate_process_tree_by_name("vMix64.exe", operation);
}

fn remove_license_backups(backups: &Path, operation: &str) -> Result<(), String> {
    if !backups.is_dir() {
        return Ok(());
    }

    let entries = std::fs::read_dir(backups).map_err(|error| format!("{operation}: {error}"))?;
    for entry in entries {
        let entry = entry.map_err(|error| format!("{operation}: {error}"))?;
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if VMIX_LICENSE_FILES
            .iter()
            .any(|file| name == *file || name.starts_with(&format!("{file}.")))
        {
            remove_file_if_exists(&entry.path(), operation)?;
        }
    }
    Ok(())
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

    #[test]
    fn vmix_registration_paths_match_program_data() {
        let program_data = PathBuf::from(r"C:\ProgramData\vMix");
        assert_eq!(
            program_data.join("es.xml"),
            PathBuf::from(r"C:\ProgramData\vMix\es.xml")
        );
        assert_eq!(
            program_data.join("s.xml"),
            PathBuf::from(r"C:\ProgramData\vMix\s.xml")
        );
        assert_eq!(
            program_data.join("backups").join("es.xml.1"),
            PathBuf::from(r"C:\ProgramData\vMix\backups\es.xml.1")
        );
    }
}
