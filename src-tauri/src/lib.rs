mod winops;

use std::path::PathBuf;

use tauri::Manager;

#[tauri::command]
fn make_network_private() -> Result<(), String> {
    winops::make_network_private()
}

#[tauri::command]
fn change_power_settings() -> Result<(), String> {
    winops::change_power_settings()
}

#[tauri::command]
fn is_elevated() -> bool {
    winops::is_elevated()
}

#[tauri::command]
fn relaunch_as_admin() -> Result<(), String> {
    winops::relaunch_as_admin()
}

#[tauri::command]
fn logout_chrome() -> Result<(), String> {
    winops::logout_chrome()
}

#[tauri::command]
fn logout_discord() -> Result<(), String> {
    winops::logout_discord()
}

#[tauri::command]
fn delete_download_directory() -> Result<(), String> {
    winops::delete_download_directory()
}

#[tauri::command]
fn disable_windows_notifications() -> Result<(), String> {
    winops::disable_windows_notifications()
}

#[tauri::command]
fn disable_aero() -> Result<(), String> {
    winops::disable_aero()
}

#[tauri::command]
fn set_solid_wallpaper() -> Result<(), String> {
    winops::set_solid_wallpaper()
}

#[tauri::command]
fn hide_desktop_icons_and_taskbar() -> Result<(), String> {
    winops::hide_desktop_icons_and_taskbar()
}

#[tauri::command]
fn restore_aero() -> Result<(), String> {
    winops::restore_aero()
}

#[tauri::command]
fn restore_desktop_icons_and_taskbar() -> Result<(), String> {
    winops::restore_desktop_icons_and_taskbar()
}

#[tauri::command]
fn list_network_adapters() -> Result<Vec<winops::NetworkAdapterInfo>, String> {
    winops::list_network_adapters()
}

#[tauri::command]
fn enable_dhcp(adapter_id: String) -> Result<(), String> {
    winops::enable_dhcp(adapter_id)
}

#[tauri::command]
fn enable_dhcp_for_connected_adapters() -> Result<(), String> {
    winops::enable_dhcp_for_connected_adapters()
}

#[tauri::command]
fn disable_sticky_keys() -> Result<(), String> {
    winops::disable_sticky_keys()
}

#[tauri::command]
fn disable_onedrive_sync() -> Result<(), String> {
    winops::disable_onedrive_sync()
}

#[tauri::command]
fn disable_delivery_optimization() -> Result<(), String> {
    winops::disable_delivery_optimization()
}

#[tauri::command]
fn defer_windows_update() -> Result<(), String> {
    winops::defer_windows_update()
}

#[tauri::command]
fn purge_obs_settings() -> Result<(), String> {
    winops::purge_obs_settings()
}

#[tauri::command]
fn purge_vmix_settings() -> Result<(), String> {
    winops::purge_vmix_settings()
}

#[tauri::command]
fn reset_vmix_registration() -> Result<(), String> {
    winops::reset_vmix_registration()
}

#[tauri::command]
fn optimize_ndi_settings() -> Result<(), String> {
    winops::optimize_ndi_settings()
}

#[tauri::command]
fn optimize_vmix_settings() -> Result<(), String> {
    winops::optimize_vmix_settings()
}

#[tauri::command]
fn optimize_nvidia_settings() -> Result<(), String> {
    winops::optimize_nvidia_settings()
}

#[tauri::command]
fn disable_call_ducking() -> Result<(), String> {
    winops::disable_call_ducking()
}

#[tauri::command]
fn set_no_sounds_scheme() -> Result<(), String> {
    winops::set_no_sounds_scheme()
}

#[tauri::command]
fn disable_telemetry() -> Result<(), String> {
    winops::disable_telemetry()
}

#[tauri::command]
fn disable_suggestions() -> Result<(), String> {
    winops::disable_suggestions()
}

#[tauri::command]
fn disable_copilot() -> Result<(), String> {
    winops::disable_copilot()
}

#[tauri::command]
fn disable_recall_and_click_to_do() -> Result<(), String> {
    winops::disable_recall_and_click_to_do()
}

#[tauri::command]
fn disable_widgets() -> Result<(), String> {
    winops::disable_widgets()
}

#[tauri::command]
fn disable_bing_search() -> Result<(), String> {
    winops::disable_bing_search()
}

#[tauri::command]
fn disable_fast_startup() -> Result<(), String> {
    winops::disable_fast_startup()
}

#[tauri::command]
fn disable_storage_sense() -> Result<(), String> {
    winops::disable_storage_sense()
}

#[tauri::command]
async fn install_winget_package(package_id: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || winops::install_winget_package(&package_id))
        .await
        .map_err(|error| format!("パッケージのインストールに失敗しました: {error}"))?
}

#[tauri::command]
async fn remove_debloat_group(app: tauri::AppHandle, group: String) -> Result<(), String> {
    let dir = debloat_dir(&app)?;
    let winget = winops::resolve_winget().ok();
    tauri::async_runtime::spawn_blocking(move || {
        winops::remove_debloat_group(&dir, &group, winget.as_deref())
    })
    .await
    .map_err(|error| format!("不要アプリの削除に失敗しました: {error}"))?
}

fn debloat_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let candidates = [
        app.path()
            .resolve("resources/debloat/apps.json", tauri::path::BaseDirectory::Resource),
        app.path()
            .resolve("debloat/apps.json", tauri::path::BaseDirectory::Resource),
    ];
    for candidate in candidates.into_iter().flatten() {
        if candidate.is_file() {
            if let Some(parent) = candidate.parent() {
                return Ok(parent.to_path_buf());
            }
        }
    }

    let dev = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("resources").join("debloat");
    if dev.join("apps.json").is_file() && dev.join("remove-apps.ps1").is_file() {
        return Ok(dev);
    }

    Err("不要アプリの削除に失敗しました: 削除スクリプトが見つかりません。".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            is_elevated,
            relaunch_as_admin,
            logout_discord,
            logout_chrome,
            delete_download_directory,
            change_power_settings,
            make_network_private,
            disable_windows_notifications,
            disable_aero,
            set_solid_wallpaper,
            hide_desktop_icons_and_taskbar,
            restore_aero,
            restore_desktop_icons_and_taskbar,
            list_network_adapters,
            enable_dhcp,
            enable_dhcp_for_connected_adapters,
            disable_sticky_keys,
            disable_onedrive_sync,
            disable_delivery_optimization,
            defer_windows_update,
            purge_obs_settings,
            purge_vmix_settings,
            reset_vmix_registration,
            optimize_ndi_settings,
            optimize_vmix_settings,
            optimize_nvidia_settings,
            disable_call_ducking,
            set_no_sounds_scheme,
            disable_telemetry,
            disable_suggestions,
            disable_copilot,
            disable_recall_and_click_to_do,
            disable_widgets,
            disable_bing_search,
            disable_fast_startup,
            disable_storage_sense,
            install_winget_package,
            remove_debloat_group
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
