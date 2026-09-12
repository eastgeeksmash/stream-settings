use std::os::windows::ffi::OsStrExt;

use windows::core::w;
use windows::Win32::UI::Shell::{IsUserAnAdmin, ShellExecuteW};
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

const RELAUNCH_OPERATION: &str = "管理者としての再起動に失敗しました";

pub fn is_elevated() -> bool {
    unsafe { IsUserAnAdmin().as_bool() }
}

pub fn relaunch_as_admin() -> Result<(), String> {
    if is_elevated() {
        return Ok(());
    }

    let exe = std::env::current_exe().map_err(|error| format!("{RELAUNCH_OPERATION}: {error}"))?;
    let path: Vec<u16> = exe
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    let result = unsafe {
        ShellExecuteW(
            None,
            w!("runas"),
            windows::core::PCWSTR(path.as_ptr()),
            None,
            None,
            SW_SHOWNORMAL,
        )
    };

    if result.0 as usize <= 32 {
        return Err(format!(
            "{RELAUNCH_OPERATION}: 管理者権限の要求が拒否されたか、起動に失敗しました。"
        ));
    }

    std::process::exit(0);
}
