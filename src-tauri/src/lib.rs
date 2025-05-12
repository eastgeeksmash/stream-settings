#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn logout_discord() -> Result<(), String> {
    // REM Discordのプロセスをキル
    // taskkill /F /IM Discord.exe /T

    // 5秒待つ
    // timeout /t 5 /nobreak

    // Discordのデータベースファイルを削除
    // del "C:\Users\%USERNAME%\AppData\Roaming\Discord\Local Storage\leveldb\*.*" /q
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, logout_discord])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
