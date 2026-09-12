use std::mem::size_of;
use std::ptr;
use std::thread;
use std::time::Duration;

use windows::core::{s, w};
use windows::Win32::Foundation::{FreeLibrary, HMODULE};
use windows::Win32::System::LibraryLoader::{
    GetProcAddress, LoadLibraryExW, LOAD_LIBRARY_SEARCH_SYSTEM32,
};

const OPERATION: &str = "NVIDIA向け最適化に失敗しました";
const MISSING_DRIVER: &str = "NVIDIAドライバーが見つかりません。";
const COLOR_WAIT: Duration = Duration::from_secs(3);

const NVAPI_MAX_PHYSICAL_GPUS: usize = 64;
const NVAPI_UNICODE_STRING_MAX: usize = 2048;
const NVDRS_VALUE_SIZE: usize = 4 + 4096;

/// NvAPI_Initialize — NVIDIA public ID
const ID_INITIALIZE: u32 = 0x0150_E828;
/// NvAPI_Unload
const ID_UNLOAD: u32 = 0xD22B_DD7E;
/// NvAPI_DRS_CreateSession
const ID_DRS_CREATE_SESSION: u32 = 0x0694_D52E;
/// NvAPI_DRS_DestroySession
const ID_DRS_DESTROY_SESSION: u32 = 0xDAD9_CFF8;
/// NvAPI_DRS_LoadSettings
const ID_DRS_LOAD_SETTINGS: u32 = 0x375D_BD6B;
/// NvAPI_DRS_SaveSettings
const ID_DRS_SAVE_SETTINGS: u32 = 0xFCBC_7E14;
/// NvAPI_DRS_GetBaseProfile
const ID_DRS_GET_BASE_PROFILE: u32 = 0xDA84_66A0;
/// NvAPI_DRS_SetSetting
const ID_DRS_SET_SETTING: u32 = 0x577D_D202;
/// NvAPI_EnumPhysicalGPUs
const ID_ENUM_PHYSICAL_GPUS: u32 = 0xE5AC_921F;
/// NvAPI_GPU_GetConnectedDisplayIds
const ID_GPU_GET_CONNECTED_DISPLAY_IDS: u32 = 0x0078_DBA2;
/// NvAPI_Disp_ColorControl
const ID_DISP_COLOR_CONTROL: u32 = 0x92F9_D80D;

/// Power management mode — NvApiDriverSettings.h
const PREFERRED_PSTATE_ID: u32 = 0x1057_EB71;
const PREFERRED_PSTATE_PREFER_MAX: u32 = 1;
/// Antialiasing - Mode
const AA_MODE_SELECTOR_ID: u32 = 0x107E_FC5B;
const AA_MODE_SELECTOR_APP_CONTROL: u32 = 0;
/// Anisotropic filtering mode
const ANISO_MODE_SELECTOR_ID: u32 = 0x10D2_BB16;
const ANISO_MODE_SELECTOR_APP: u32 = 0;

const NVDRS_DWORD_TYPE: u32 = 0;
const NVDRS_CURRENT_PROFILE_LOCATION: u32 = 0;

const NVAPI_OK: i32 = 0;
const NVAPI_ERROR: i32 = -1;
const NVAPI_LIBRARY_NOT_FOUND: i32 = -2;
const NVAPI_NO_IMPLEMENTATION: i32 = -3;
const NVAPI_API_NOT_INITIALIZED: i32 = -4;
const NVAPI_INVALID_ARGUMENT: i32 = -5;
const NVAPI_NVIDIA_DEVICE_NOT_FOUND: i32 = -6;
const NVAPI_INCOMPATIBLE_STRUCT_VERSION: i32 = -9;
const NVAPI_NOT_SUPPORTED: i32 = -104;
const NVAPI_INVALID_USER_PRIVILEGE: i32 = -137;

const NV_COLOR_CMD_GET: u8 = 1;
const NV_COLOR_CMD_SET: u8 = 2;
const NV_COLOR_FORMAT_RGB: u8 = 0;
const NV_COLOR_FORMAT_YUV422: u8 = 1;
const NV_COLOR_COLORIMETRY_DEFAULT: u8 = 0xFE;
const NV_DYNAMIC_RANGE_AUTO: u8 = 0xFF;
const NV_BPC_DEFAULT: u32 = 0;
const NV_COLOR_SELECTION_POLICY_USER: u32 = 0;
const NV_DESKTOP_COLOR_DEPTH_DEFAULT: u32 = 0;

type QueryInterface = unsafe extern "C" fn(u32) -> *const std::ffi::c_void;
type NvStatusFn = unsafe extern "C" fn() -> i32;
type DrsCreateSessionFn = unsafe extern "C" fn(*mut *mut std::ffi::c_void) -> i32;
type DrsSessionFn = unsafe extern "C" fn(*mut std::ffi::c_void) -> i32;
type DrsGetBaseProfileFn =
    unsafe extern "C" fn(*mut std::ffi::c_void, *mut *mut std::ffi::c_void) -> i32;
type DrsSetSettingFn =
    unsafe extern "C" fn(*mut std::ffi::c_void, *mut std::ffi::c_void, *mut NvDrsSetting) -> i32;
type EnumPhysicalGpusFn = unsafe extern "C" fn(*mut *mut std::ffi::c_void, *mut u32) -> i32;
type GetConnectedDisplayIdsFn =
    unsafe extern "C" fn(*mut std::ffi::c_void, *mut NvGpuDisplayIds, *mut u32, u32) -> i32;
type DispColorControlFn = unsafe extern "C" fn(u32, *mut NvColorData) -> i32;

#[repr(C)]
struct NvDrsSetting {
    version: u32,
    setting_name: [u16; NVAPI_UNICODE_STRING_MAX],
    setting_id: u32,
    setting_type: u32,
    setting_location: u32,
    is_current_predefined: u32,
    is_predefined_valid: u32,
    predefined: [u8; NVDRS_VALUE_SIZE],
    current: [u8; NVDRS_VALUE_SIZE],
}

#[repr(C)]
struct NvGpuDisplayIds {
    version: u32,
    connector_type: u32,
    display_id: u32,
    flags: u32,
}

#[repr(C)]
struct NvColorDataInner {
    color_format: u8,
    colorimetry: u8,
    dynamic_range: u8,
    bpc: u32,
    color_selection_policy: u32,
    depth: u32,
}

#[repr(C)]
struct NvColorData {
    version: u32,
    size: u16,
    cmd: u8,
    data: NvColorDataInner,
}

fn nvapi_version<T>(version: u32) -> u32 {
    size_of::<T>() as u32 | (version << 16)
}

fn status_message(status: i32) -> String {
    match status {
        NVAPI_OK => "成功しました。".to_string(),
        NVAPI_LIBRARY_NOT_FOUND | NVAPI_NVIDIA_DEVICE_NOT_FOUND => MISSING_DRIVER.to_string(),
        NVAPI_NO_IMPLEMENTATION | NVAPI_NOT_SUPPORTED => {
            "このドライバーまたはモニターでは未対応です。".to_string()
        }
        NVAPI_API_NOT_INITIALIZED => "NVAPIが初期化されていません。".to_string(),
        NVAPI_INVALID_ARGUMENT => "引数が不正です。".to_string(),
        NVAPI_INCOMPATIBLE_STRUCT_VERSION => "構造体の版が合いません。".to_string(),
        NVAPI_INVALID_USER_PRIVILEGE => {
            "権限が不足しています。管理者として実行してください。".to_string()
        }
        NVAPI_ERROR => "処理に失敗しました。".to_string(),
        other => format!("エラーコード {other}"),
    }
}

fn status_error(status: i32) -> String {
    format!("{OPERATION}: {}", status_message(status))
}

fn check(status: i32) -> Result<(), String> {
    if status == NVAPI_OK {
        Ok(())
    } else {
        Err(status_error(status))
    }
}

fn resolve<T: Copy>(query: QueryInterface, id: u32, name: &str) -> Result<T, String> {
    let pointer = unsafe { query(id) };
    if pointer.is_null() {
        return Err(format!(
            "{OPERATION}: NVAPIの関数を解決できませんでした ({name})。"
        ));
    }
    Ok(unsafe { ptr::read(&pointer as *const *const std::ffi::c_void as *const T) })
}

struct NvApi {
    module: HMODULE,
    initialized: bool,
    unload: NvStatusFn,
    drs_create_session: DrsCreateSessionFn,
    drs_destroy_session: DrsSessionFn,
    drs_load_settings: DrsSessionFn,
    drs_save_settings: DrsSessionFn,
    drs_get_base_profile: DrsGetBaseProfileFn,
    drs_set_setting: DrsSetSettingFn,
    enum_physical_gpus: EnumPhysicalGpusFn,
    get_connected_display_ids: GetConnectedDisplayIdsFn,
    disp_color_control: DispColorControlFn,
}

impl NvApi {
    fn load() -> Result<Self, String> {
        let module = unsafe {
            LoadLibraryExW(w!("nvapi64.dll"), None, LOAD_LIBRARY_SEARCH_SYSTEM32)
                .map_err(|_| format!("{OPERATION}: {MISSING_DRIVER}"))?
        };
        let proc = unsafe { GetProcAddress(module, s!("nvapi_QueryInterface")) };
        let Some(proc) = proc else {
            unsafe {
                let _ = FreeLibrary(module);
            }
            return Err(format!("{OPERATION}: {MISSING_DRIVER}"));
        };
        let query: QueryInterface = unsafe { std::mem::transmute(proc) };

        let resolved = (|| {
            Ok(Self {
                module,
                initialized: false,
                unload: resolve(query, ID_UNLOAD, "Unload")?,
                drs_create_session: resolve(query, ID_DRS_CREATE_SESSION, "DRS_CreateSession")?,
                drs_destroy_session: resolve(query, ID_DRS_DESTROY_SESSION, "DRS_DestroySession")?,
                drs_load_settings: resolve(query, ID_DRS_LOAD_SETTINGS, "DRS_LoadSettings")?,
                drs_save_settings: resolve(query, ID_DRS_SAVE_SETTINGS, "DRS_SaveSettings")?,
                drs_get_base_profile: resolve(query, ID_DRS_GET_BASE_PROFILE, "DRS_GetBaseProfile")?,
                drs_set_setting: resolve(query, ID_DRS_SET_SETTING, "DRS_SetSetting")?,
                enum_physical_gpus: resolve(query, ID_ENUM_PHYSICAL_GPUS, "EnumPhysicalGPUs")?,
                get_connected_display_ids: resolve(
                    query,
                    ID_GPU_GET_CONNECTED_DISPLAY_IDS,
                    "GPU_GetConnectedDisplayIds",
                )?,
                disp_color_control: resolve(query, ID_DISP_COLOR_CONTROL, "Disp_ColorControl")?,
            })
        })();
        let mut api = match resolved {
            Ok(api) => api,
            Err(error) => {
                unsafe {
                    let _ = FreeLibrary(module);
                }
                return Err(error);
            }
        };

        let initialize: NvStatusFn = match resolve(query, ID_INITIALIZE, "Initialize") {
            Ok(function) => function,
            Err(error) => return Err(error),
        };
        check(unsafe { initialize() })?;
        api.initialized = true;
        Ok(api)
    }

    fn apply_drs_settings(&self) -> Result<(), String> {
        let mut session = ptr::null_mut();
        check(unsafe { (self.drs_create_session)(&mut session) })?;

        let result = (|| {
            check(unsafe { (self.drs_load_settings)(session) })?;
            let mut profile = ptr::null_mut();
            check(unsafe { (self.drs_get_base_profile)(session, &mut profile) })?;
            for (setting_id, value) in [
                (PREFERRED_PSTATE_ID, PREFERRED_PSTATE_PREFER_MAX),
                (AA_MODE_SELECTOR_ID, AA_MODE_SELECTOR_APP_CONTROL),
                (ANISO_MODE_SELECTOR_ID, ANISO_MODE_SELECTOR_APP),
            ] {
                let mut setting = dword_setting(setting_id, value);
                check(unsafe { (self.drs_set_setting)(session, profile, &mut setting) })?;
            }
            check(unsafe { (self.drs_save_settings)(session) })
        })();

        unsafe {
            let _ = (self.drs_destroy_session)(session);
        }
        result
    }

    fn connected_display_ids(&self) -> Result<Vec<u32>, String> {
        let mut gpus = [ptr::null_mut(); NVAPI_MAX_PHYSICAL_GPUS];
        let mut gpu_count = 0u32;
        check(unsafe { (self.enum_physical_gpus)(gpus.as_mut_ptr(), &mut gpu_count) })?;

        let mut display_ids = Vec::new();
        for gpu in gpus.iter().take(gpu_count as usize) {
            let mut count = 0u32;
            let status =
                unsafe { (self.get_connected_display_ids)(*gpu, ptr::null_mut(), &mut count, 0) };
            if status != NVAPI_OK || count == 0 {
                continue;
            }

            let mut displays = (0..count)
                .map(|_| NvGpuDisplayIds {
                    version: nvapi_version::<NvGpuDisplayIds>(3),
                    connector_type: 0,
                    display_id: 0,
                    flags: 0,
                })
                .collect::<Vec<_>>();
            if unsafe {
                (self.get_connected_display_ids)(*gpu, displays.as_mut_ptr(), &mut count, 0)
            } != NVAPI_OK
            {
                continue;
            }
            display_ids.extend(
                displays
                    .into_iter()
                    .take(count as usize)
                    .map(|display| display.display_id)
                    .filter(|id| *id != 0),
            );
        }
        Ok(display_ids)
    }

    fn set_color_format(&self, display_id: u32, color_format: u8) -> Result<(), String> {
        let mut color = current_or_default_color(self, display_id);
        color.version = nvapi_version::<NvColorData>(5);
        color.size = size_of::<NvColorData>() as u16;
        color.cmd = NV_COLOR_CMD_SET;
        color.data.color_format = color_format;
        color.data.color_selection_policy = NV_COLOR_SELECTION_POLICY_USER;
        check(unsafe { (self.disp_color_control)(display_id, &mut color) })
    }

    fn switch_color_formats(&self) -> Result<(), String> {
        let display_ids = self.connected_display_ids()?;
        if display_ids.is_empty() {
            return Ok(());
        }

        for display_id in &display_ids {
            let _ = self.set_color_format(*display_id, NV_COLOR_FORMAT_RGB);
        }

        thread::sleep(COLOR_WAIT);

        let mut successes = 0usize;
        for display_id in &display_ids {
            if self
                .set_color_format(*display_id, NV_COLOR_FORMAT_YUV422)
                .is_ok()
            {
                successes += 1;
            }
        }
        if successes == 0 {
            return Err(format!(
                "{OPERATION}: YCbCr 4:2:2 に切り替えられるモニターがありません。"
            ));
        }
        Ok(())
    }
}

impl Drop for NvApi {
    fn drop(&mut self) {
        if self.initialized {
            unsafe {
                let _ = (self.unload)();
            }
        }
        unsafe {
            let _ = FreeLibrary(self.module);
        }
    }
}

fn write_u32(buffer: &mut [u8], value: u32) {
    buffer[..4].copy_from_slice(&value.to_le_bytes());
}

fn dword_setting(setting_id: u32, value: u32) -> NvDrsSetting {
    let mut setting = unsafe { std::mem::zeroed::<NvDrsSetting>() };
    setting.version = nvapi_version::<NvDrsSetting>(1);
    setting.setting_id = setting_id;
    setting.setting_type = NVDRS_DWORD_TYPE;
    setting.setting_location = NVDRS_CURRENT_PROFILE_LOCATION;
    write_u32(&mut setting.predefined, value);
    write_u32(&mut setting.current, value);
    setting
}

fn empty_color_data() -> NvColorData {
    NvColorData {
        version: nvapi_version::<NvColorData>(5),
        size: size_of::<NvColorData>() as u16,
        cmd: NV_COLOR_CMD_GET,
        data: NvColorDataInner {
            color_format: NV_COLOR_FORMAT_RGB,
            colorimetry: NV_COLOR_COLORIMETRY_DEFAULT,
            dynamic_range: NV_DYNAMIC_RANGE_AUTO,
            bpc: NV_BPC_DEFAULT,
            color_selection_policy: NV_COLOR_SELECTION_POLICY_USER,
            depth: NV_DESKTOP_COLOR_DEPTH_DEFAULT,
        },
    }
}

fn current_or_default_color(api: &NvApi, display_id: u32) -> NvColorData {
    let mut color = empty_color_data();
    color.cmd = NV_COLOR_CMD_GET;
    if unsafe { (api.disp_color_control)(display_id, &mut color) } == NVAPI_OK {
        color
    } else {
        empty_color_data()
    }
}

pub fn optimize_nvidia_settings() -> Result<(), String> {
    let api = NvApi::load()?;
    api.apply_drs_settings()?;
    api.switch_color_formats()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn function_ids_match_public_nvapi_headers() {
        assert_eq!(ID_INITIALIZE, 0x0150E828);
        assert_eq!(ID_UNLOAD, 0xD22BDD7E);
        assert_eq!(ID_DRS_CREATE_SESSION, 0x0694D52E);
        assert_eq!(ID_DRS_DESTROY_SESSION, 0xDAD9CFF8);
        assert_eq!(ID_DRS_LOAD_SETTINGS, 0x375DBD6B);
        assert_eq!(ID_DRS_SAVE_SETTINGS, 0xFCBC7E14);
        assert_eq!(ID_DRS_GET_BASE_PROFILE, 0xDA8466A0);
        assert_eq!(ID_DRS_SET_SETTING, 0x577DD202);
        assert_eq!(ID_ENUM_PHYSICAL_GPUS, 0xE5AC921F);
        assert_eq!(ID_GPU_GET_CONNECTED_DISPLAY_IDS, 0x0078DBA2);
        assert_eq!(ID_DISP_COLOR_CONTROL, 0x92F9D80D);
    }

    #[test]
    fn setting_ids_match_nvapi_driver_settings_header() {
        assert_eq!(PREFERRED_PSTATE_ID, 0x1057EB71);
        assert_eq!(PREFERRED_PSTATE_PREFER_MAX, 1);
        assert_eq!(AA_MODE_SELECTOR_ID, 0x107EFC5B);
        assert_eq!(AA_MODE_SELECTOR_APP_CONTROL, 0);
        assert_eq!(ANISO_MODE_SELECTOR_ID, 0x10D2BB16);
        assert_eq!(ANISO_MODE_SELECTOR_APP, 0);
    }

    #[test]
    fn maps_nvapi_status_to_japanese() {
        assert_eq!(status_message(NVAPI_LIBRARY_NOT_FOUND), MISSING_DRIVER);
        assert_eq!(status_message(NVAPI_NVIDIA_DEVICE_NOT_FOUND), MISSING_DRIVER);
        assert!(status_message(NVAPI_INVALID_USER_PRIVILEGE).contains("管理者として実行"));
        assert_eq!(status_message(-999), "エラーコード -999");
        assert!(status_error(NVAPI_ERROR).starts_with(OPERATION));
    }

    #[test]
    fn struct_versions_encode_size_and_revision() {
        assert_eq!(size_of::<NvGpuDisplayIds>(), 16);
        assert_eq!(nvapi_version::<NvGpuDisplayIds>(3), 16 | (3 << 16));
        assert_eq!(size_of::<NvColorData>(), 24);
        assert_eq!(nvapi_version::<NvColorData>(5), 24 | (5 << 16));
        assert_eq!(size_of::<NvDrsSetting>(), 12_320);
        assert_eq!(nvapi_version::<NvDrsSetting>(1), 12_320 | (1 << 16));
    }

    #[test]
    fn dword_setting_writes_current_value() {
        let setting = dword_setting(PREFERRED_PSTATE_ID, PREFERRED_PSTATE_PREFER_MAX);
        assert_eq!(setting.setting_id, PREFERRED_PSTATE_ID);
        assert_eq!(setting.setting_type, NVDRS_DWORD_TYPE);
        assert_eq!(
            u32::from_le_bytes(setting.current[..4].try_into().unwrap()),
            PREFERRED_PSTATE_PREFER_MAX
        );
    }

    #[test]
    fn nvapi_loader_accepts_missing_or_present_driver() {
        match NvApi::load() {
            Ok(api) => {
                let ids = api
                    .connected_display_ids()
                    .expect("接続ディスプレイの列挙に失敗しました");
                let _ = ids.len();
            }
            Err(message) => {
                assert!(
                    message.contains(MISSING_DRIVER),
                    "想定外の読み込み失敗: {message}"
                );
            }
        }
    }

    #[test]
    #[ignore]
    fn apply_nvidia_settings_on_this_machine() {
        optimize_nvidia_settings().expect("NVIDIA向け最適化に失敗しました");
    }
}
