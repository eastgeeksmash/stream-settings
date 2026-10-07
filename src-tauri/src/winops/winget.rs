use std::env;
use std::fs;
use std::os::windows::process::CommandExt;
use std::path::PathBuf;
use std::process::{Command, Output};

const CREATE_NO_WINDOW: u32 = 0x0800_0000;
const OPERATION: &str = "パッケージのインストールに失敗しました";
/// winget APPINSTALLER_CLI_ERROR_UPDATE_NOT_APPLICABLE: already installed, no upgrade.
const ALREADY_PRESENT: i32 = -1978335189;

pub const WINGET_PACKAGE_IDS: &[&str] = &[
    "LocalSend.LocalSend",
    "Google.Chrome",
    "Microsoft.Coreutils",
    "Gyan.FFmpeg",
];

pub fn install_winget_package(package_id: &str) -> Result<(), String> {
    if !WINGET_PACKAGE_IDS.contains(&package_id) {
        return Err(format!("{OPERATION}: 対象外のパッケージです。"));
    }

    let winget = resolve_winget()?;
    let output = Command::new(&winget)
        .args([
            "install",
            "--id",
            package_id,
            "-e",
            "--accept-package-agreements",
            "--accept-source-agreements",
            "--disable-interactivity",
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map_err(|error| format!("{OPERATION}: {error}"))?;

    if install_exit_is_success(output.status.code().unwrap_or(-1)) {
        return Ok(());
    }

    Err(format!("{OPERATION}: {}", output_detail(&output)))
}

pub fn resolve_winget() -> Result<PathBuf, String> {
    let mut matches = Vec::new();
    for root in windows_apps_roots() {
        let Ok(entries) = fs::read_dir(&root) else {
            continue;
        };
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if !is_app_installer_package_dir(&name) {
                continue;
            }
            let exe = entry.path().join("winget.exe");
            if exe.is_file() {
                matches.push(exe);
            }
        }
    }
    matches.sort();
    if let Some(path) = matches.pop() {
        return Ok(path);
    }

    if let Some(path) = search_path("winget.exe") {
        return Ok(path);
    }

    Err(format!(
        "{OPERATION}: winget.exe が見つかりません。App Installer をインストールしてください。"
    ))
}

pub fn install_exit_is_success(code: i32) -> bool {
    code == 0 || code == ALREADY_PRESENT
}

pub fn is_app_installer_package_dir(name: &str) -> bool {
    name.starts_with("Microsoft.DesktopAppInstaller_")
}

fn windows_apps_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    if let Some(program_files) = env::var_os("ProgramFiles") {
        roots.push(PathBuf::from(program_files).join("WindowsApps"));
    }
    if let Some(local) = env::var_os("LOCALAPPDATA") {
        roots.push(PathBuf::from(local).join(r"Microsoft\WindowsApps"));
    }
    roots
}

fn search_path(file_name: &str) -> Option<PathBuf> {
    let paths = env::var_os("PATH")?;
    env::split_paths(&paths)
        .map(|dir| dir.join(file_name))
        .find(|path| path.is_file())
}

fn output_detail(output: &Output) -> String {
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let text = if stderr.trim().is_empty() {
        stdout.trim()
    } else {
        stderr.trim()
    };
    let lines: Vec<&str> = text.lines().rev().take(12).collect();
    let detail = lines.into_iter().rev().collect::<Vec<_>>().join("\n");
    if detail.is_empty() {
        format!("終了コード {}", output.status.code().unwrap_or(-1))
    } else {
        detail
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn package_ids_match_issue() {
        assert_eq!(
            WINGET_PACKAGE_IDS,
            [
                "LocalSend.LocalSend",
                "Google.Chrome",
                "Microsoft.Coreutils",
                "Gyan.FFmpeg",
            ]
        );
    }

    #[test]
    fn already_installed_exit_is_success() {
        assert!(install_exit_is_success(0));
        assert!(install_exit_is_success(ALREADY_PRESENT));
        assert!(!install_exit_is_success(1));
    }

    #[test]
    fn app_installer_directory_filter() {
        assert!(is_app_installer_package_dir(
            "Microsoft.DesktopAppInstaller_1.26.430.0_x64__8wekyb3d8bbwe"
        ));
        assert!(!is_app_installer_package_dir("Microsoft.WindowsStore_8wekyb3d8bbwe"));
    }
}
