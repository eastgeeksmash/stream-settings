use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED};

use super::error::from_windows_error;

pub struct ComInitializer;

impl ComInitializer {
    pub fn new(operation: &str) -> Result<Self, String> {
        let hr = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED) };
        if hr.is_err() {
            return Err(from_windows_error(
                operation,
                windows::core::Error::from(hr),
            ));
        }
        Ok(Self)
    }
}

impl Drop for ComInitializer {
    fn drop(&mut self) {
        unsafe { CoUninitialize() };
    }
}
