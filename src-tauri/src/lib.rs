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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            logout_discord,
            logout_chrome,
            delete_download_directory,
            change_power_settings,
            make_network_private,
            disable_windows_notifications
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
