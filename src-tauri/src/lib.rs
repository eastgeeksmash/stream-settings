mod winops;

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
            optimize_ndi_settings,
            optimize_vmix_settings,
            optimize_nvidia_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
