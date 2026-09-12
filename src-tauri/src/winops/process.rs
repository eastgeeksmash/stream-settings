use std::env;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

use windows::Win32::Foundation::CloseHandle;
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS,
};
use windows::Win32::System::Threading::{
    GetCurrentProcessId, OpenProcess, TerminateProcess, PROCESS_TERMINATE,
};

use super::error::from_windows_error;

const OPERATION: &str = "Discordプロセスの終了に失敗しました";
pub const DISCORD_PROCESS_NAME: &str = "Discord.exe";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProcessRecord {
    pub pid: u32,
    pub parent_pid: u32,
    pub name: String,
}

pub fn process_name_matches(exe_name: &str, target: &str) -> bool {
    Path::new(exe_name)
        .file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.eq_ignore_ascii_case(target))
        .unwrap_or(false)
}

pub fn collect_process_tree(entries: &[ProcessRecord], roots: &[u32]) -> Vec<u32> {
    let mut result = roots.to_vec();
    let mut added = true;
    while added {
        added = false;
        for entry in entries {
            if result.contains(&entry.parent_pid) && !result.contains(&entry.pid) {
                result.push(entry.pid);
                added = true;
            }
        }
    }
    result
}

pub fn logout_discord() -> Result<(), String> {
    let terminated = terminate_discord_tree()?;
    if terminated {
        thread::sleep(Duration::from_secs(5));
    }

    let home_dir = env::var("USERPROFILE").map_err(|error| error.to_string())?;
    let leveldb_path = PathBuf::from(home_dir)
        .join("AppData")
        .join("Roaming")
        .join("Discord")
        .join("Local Storage")
        .join("leveldb");

    if leveldb_path.exists() {
        for entry in std::fs::read_dir(&leveldb_path).map_err(|error| error.to_string())? {
            let entry = entry.map_err(|error| error.to_string())?;
            let path = entry.path();
            if path.is_file() {
                std::fs::remove_file(&path).map_err(|error| {
                    format!("Discordデータベースファイルの削除に失敗しました: {error}")
                })?;
            }
        }
    }

    Ok(())
}

fn terminate_discord_tree() -> Result<bool, String> {
    let entries = snapshot_processes()?;
    let current_pid = unsafe { GetCurrentProcessId() };
    let roots: Vec<u32> = entries
        .iter()
        .filter(|entry| process_name_matches(&entry.name, DISCORD_PROCESS_NAME))
        .map(|entry| entry.pid)
        .collect();

    if roots.is_empty() {
        return Ok(false);
    }

    let targets = collect_process_tree(&entries, &roots)
        .into_iter()
        .filter(|pid| *pid != current_pid)
        .collect::<Vec<_>>();

    let mut failures = Vec::new();
    for pid in targets.iter().rev() {
        if let Err(error) = terminate_pid(*pid) {
            failures.push(error);
        }
    }

    if let Some(error) = failures.into_iter().next() {
        return Err(error);
    }

    Ok(true)
}

pub(super) fn snapshot_processes() -> Result<Vec<ProcessRecord>, String> {
    let snapshot = unsafe {
        CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)
            .map_err(|error| from_windows_error(OPERATION, error))?
    };

    let mut entry = PROCESSENTRY32W {
        dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
        ..Default::default()
    };

    let mut records = Vec::new();
    let first = unsafe { Process32FirstW(snapshot, &mut entry) };
    if first.is_err() {
        unsafe {
            let _ = CloseHandle(snapshot);
        }
        return first
            .map(|_| Vec::new())
            .map_err(|error| from_windows_error(OPERATION, error));
    }

    loop {
        records.push(ProcessRecord {
            pid: entry.th32ProcessID,
            parent_pid: entry.th32ParentProcessID,
            name: wide_to_string(&entry.szExeFile),
        });

        if unsafe { Process32NextW(snapshot, &mut entry) }.is_err() {
            break;
        }
    }

    unsafe {
        let _ = CloseHandle(snapshot);
    }
    Ok(records)
}

fn terminate_pid(pid: u32) -> Result<(), String> {
    let handle = match unsafe { OpenProcess(PROCESS_TERMINATE, false, pid) } {
        Ok(handle) => handle,
        Err(error) => {
            if is_not_found(&error) {
                return Ok(());
            }
            return Err(from_windows_error(OPERATION, error));
        }
    };

    let result = unsafe { TerminateProcess(handle, 1) };
    unsafe {
        let _ = CloseHandle(handle);
    }
    result.map_err(|error| from_windows_error(OPERATION, error))
}

fn is_not_found(error: &windows::core::Error) -> bool {
    let code = super::error::win32_code_from_hresult(error.code().0);
    matches!(code, 2 | 87 | 1168)
}

fn wide_to_string(value: &[u16]) -> String {
    let end = value.iter().position(|c| *c == 0).unwrap_or(value.len());
    String::from_utf16_lossy(&value[..end])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_name_match_is_case_insensitive() {
        assert!(process_name_matches("Discord.exe", DISCORD_PROCESS_NAME));
        assert!(process_name_matches("discord.exe", DISCORD_PROCESS_NAME));
        assert!(process_name_matches("DISCORD.EXE", DISCORD_PROCESS_NAME));
        assert!(!process_name_matches("chrome.exe", DISCORD_PROCESS_NAME));
    }

    #[test]
    fn process_tree_includes_descendants() {
        let entries = vec![
            ProcessRecord {
                pid: 10,
                parent_pid: 1,
                name: "Discord.exe".into(),
            },
            ProcessRecord {
                pid: 11,
                parent_pid: 10,
                name: "Discord.exe".into(),
            },
            ProcessRecord {
                pid: 12,
                parent_pid: 11,
                name: "Update.exe".into(),
            },
            ProcessRecord {
                pid: 20,
                parent_pid: 1,
                name: "unrelated.exe".into(),
            },
        ];

        let tree = collect_process_tree(&entries, &[10]);
        assert_eq!(tree, vec![10, 11, 12]);
    }
}
