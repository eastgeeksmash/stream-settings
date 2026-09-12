use std::path::{Path, PathBuf};

use windows::core::GUID;
use windows::Win32::System::Com::CoTaskMemFree;
use windows::Win32::UI::Shell::{
    FOLDERID_Downloads, FOLDERID_LocalAppData, FOLDERID_ProgramData, FOLDERID_RoamingAppData,
    SHGetKnownFolderPath,
    KF_FLAG_DEFAULT,
};

use super::com::ComInitializer;
use super::error::from_windows_error;

const OPERATION: &str = "ダウンロードディレクトリの削除に失敗しました";

pub fn prefer_known_folder(known: Option<PathBuf>) -> Result<PathBuf, String> {
    known
        .filter(|path| !path.as_os_str().is_empty())
        .ok_or_else(|| "ダウンロードフォルダのパスを取得できませんでした。".to_string())
}

pub fn delete_download_directory() -> Result<(), String> {
    let downloads_dir = prefer_known_folder(Some(known_downloads_path()?))?;
    if !downloads_dir.exists() {
        return Ok(());
    }

    std::fs::remove_dir_all(&downloads_dir).map_err(|error| format!("{OPERATION}: {error}"))?;
    std::fs::create_dir(&downloads_dir)
        .map_err(|error| format!("ダウンロードディレクトリの再作成に失敗しました: {error}"))?;
    Ok(())
}

pub fn local_app_data(operation: &str) -> Result<PathBuf, String> {
    known_folder(&FOLDERID_LocalAppData, operation)
}

pub fn roaming_app_data(operation: &str) -> Result<PathBuf, String> {
    known_folder(&FOLDERID_RoamingAppData, operation)
}

pub fn program_data(operation: &str) -> Result<PathBuf, String> {
    known_folder(&FOLDERID_ProgramData, operation)
}

pub fn remove_tree_if_exists(path: &Path, operation: &str) -> Result<(), String> {
    if !path.exists() {
        return Ok(());
    }
    std::fs::remove_dir_all(path).map_err(|error| format!("{operation}: {error}"))
}

pub fn remove_file_if_exists(path: &Path, operation: &str) -> Result<(), String> {
    if !path.exists() {
        return Ok(());
    }
    std::fs::remove_file(path).map_err(|error| format!("{operation}: {error}"))
}

fn known_downloads_path() -> Result<PathBuf, String> {
    known_folder(&FOLDERID_Downloads, OPERATION)
}

fn known_folder(id: &GUID, operation: &str) -> Result<PathBuf, String> {
    let _com = ComInitializer::new(operation)?;
    let pwstr = unsafe {
        SHGetKnownFolderPath(id, KF_FLAG_DEFAULT, None)
            .map_err(|error| from_windows_error(operation, error))?
    };

    let path = unsafe { pwstr.to_string() }.map_err(|error| format!("{operation}: {error}"))?;
    unsafe { CoTaskMemFree(Some(pwstr.as_ptr().cast())) };
    Ok(PathBuf::from(path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefers_non_empty_known_folder_path() {
        let path = prefer_known_folder(Some(PathBuf::from(r"D:\Redirected\Downloads"))).unwrap();
        assert_eq!(path, PathBuf::from(r"D:\Redirected\Downloads"));
    }

    #[test]
    fn rejects_missing_or_empty_known_folder_path() {
        assert!(prefer_known_folder(None).is_err());
        assert!(prefer_known_folder(Some(PathBuf::new())).is_err());
    }
}
