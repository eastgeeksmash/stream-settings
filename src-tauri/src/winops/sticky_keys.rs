use windows::Win32::UI::Accessibility::{
    SKF_CONFIRMHOTKEY, SKF_HOTKEYACTIVE, SKF_STICKYKEYSON, STICKYKEYS, STICKYKEYS_FLAGS,
};
use windows::Win32::UI::WindowsAndMessaging::{
    SystemParametersInfoW, SPIF_SENDCHANGE, SPIF_UPDATEINIFILE, SPI_GETSTICKYKEYS,
    SPI_SETSTICKYKEYS,
};

use super::error::from_windows_error;

const OPERATION: &str = "固定キー設定の無効化に失敗しました";

pub fn disable_sticky_keys() -> Result<(), String> {
    let mut keys = STICKYKEYS {
        cbSize: std::mem::size_of::<STICKYKEYS>() as u32,
        dwFlags: STICKYKEYS_FLAGS(0),
    };

    unsafe {
        SystemParametersInfoW(
            SPI_GETSTICKYKEYS,
            keys.cbSize,
            Some((&mut keys as *mut STICKYKEYS).cast()),
            Default::default(),
        )
        .map_err(|error| from_windows_error(OPERATION, error))?;

        keys.dwFlags = STICKYKEYS_FLAGS(
            keys.dwFlags.0 & !(SKF_STICKYKEYSON.0 | SKF_HOTKEYACTIVE.0 | SKF_CONFIRMHOTKEY.0),
        );

        SystemParametersInfoW(
            SPI_SETSTICKYKEYS,
            keys.cbSize,
            Some((&mut keys as *mut STICKYKEYS).cast()),
            SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
        )
        .map_err(|error| from_windows_error(OPERATION, error))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sticky_key_disable_clears_hotkey_flags() {
        let flags = SKF_STICKYKEYSON.0 | SKF_HOTKEYACTIVE.0 | SKF_CONFIRMHOTKEY.0 | 0x10;
        assert_eq!(
            flags & !(SKF_STICKYKEYSON.0 | SKF_HOTKEYACTIVE.0 | SKF_CONFIRMHOTKEY.0),
            0x10
        );
    }
}
