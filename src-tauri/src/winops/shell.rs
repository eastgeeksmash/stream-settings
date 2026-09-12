use windows::core::w;
use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

const OPERATION: &str = "Chromeのログアウトに失敗しました";
pub const GOOGLE_LOGOUT_URL: &str = "http://accounts.google.com/logout";

pub fn logout_chrome() -> Result<(), String> {
    let url: Vec<u16> = GOOGLE_LOGOUT_URL
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();
    let result = unsafe {
        ShellExecuteW(
            None,
            w!("open"),
            windows::core::PCWSTR(url.as_ptr()),
            None,
            None,
            SW_SHOWNORMAL,
        )
    };

    if result.0 as usize <= 32 {
        return Err(format!(
            "{OPERATION}: 既定ブラウザの起動に失敗しました (コード {})",
            result.0 as isize
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logout_url_matches_previous_implementation() {
        assert_eq!(GOOGLE_LOGOUT_URL, "http://accounts.google.com/logout");
    }
}
