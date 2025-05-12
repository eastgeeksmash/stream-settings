#[tauri::command]
fn make_network_private() -> Result<(), String> {
    use std::process::Command;
    
    let output = Command::new("powershell")
        .args(&[
            "-Command",
            "Get-NetConnectionProfile | Where-Object NetworkCategory -eq 'Public' | Set-NetConnectionProfile -NetworkCategory Private"
        ])
        .output()
        .map_err(|e| e.to_string())?;
    
    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(format!("ネットワーク設定の変更に失敗しました: {}", error_message));
    }
    
    Ok(())
}

#[tauri::command]
fn change_power_settings() -> Result<(), String> {
    use std::process::Command;
    
    // 高パフォーマンスモードに変更
    let power_scheme = Command::new("powershell")
        .args(&[
            "-Command",
            "powercfg /setactive 8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c"
        ])
        .output()
        .map_err(|e| e.to_string())?;
    
    if !power_scheme.status.success() {
        let error_message = String::from_utf8_lossy(&power_scheme.stderr);
        return Err(format!("電源プランの変更に失敗しました: {}", error_message));
    }
    
    // スリープを無効化
    let disable_sleep = Command::new("powershell")
        .args(&[
            "-Command",
            "powercfg /change standby-timeout-ac 0"
        ])
        .output()
        .map_err(|e| e.to_string())?;
    
    if !disable_sleep.status.success() {
        let error_message = String::from_utf8_lossy(&disable_sleep.stderr);
        return Err(format!("スリープ設定の変更に失敗しました: {}", error_message));
    }
    
    // 電源ボタンの動作を「何もしない」に設定
    let power_button = Command::new("powershell")
        .args(&[
            "-Command",
            "powercfg -setacvalueindex SCHEME_CURRENT 4f971e89-eebd-4455-a8de-9e59040e7347 7648efa3-dd9c-4e3e-b566-50f929386280 0"
        ])
        .output()
        .map_err(|e| e.to_string())?;
    
    if !power_button.status.success() {
        let error_message = String::from_utf8_lossy(&power_button.stderr);
        return Err(format!("電源ボタン設定の変更に失敗しました: {}", error_message));
    }
    
    // 設定を適用
    let apply_settings = Command::new("powershell")
        .args(&[
            "-Command",
            "powercfg -setactive scheme_current"
        ])
        .output()
        .map_err(|e| e.to_string())?;
    
    if !apply_settings.status.success() {
        let error_message = String::from_utf8_lossy(&apply_settings.stderr);
        return Err(format!("電源設定の適用に失敗しました: {}", error_message));
    }
    Ok(())
}

#[tauri::command]
fn logout_chrome() -> Result<(), String> {
    use std::process::Command;
    
    let output = Command::new("cmd")
        .args(&["/C", "start", "http://accounts.google.com/logout"])
        .output()
        .map_err(|e| e.to_string())?;
    
    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Chromeのログアウトに失敗しました: {}", error_message));
    }
    Ok(())
}

#[tauri::command]
fn logout_discord() -> Result<(), String> {
    use std::process::Command;
    use std::thread;
    use std::time::Duration;
    use std::path::PathBuf;
    use std::env;
    
    // Discordのプロセスをキル
    let kill_output = Command::new("taskkill")
        .args(&["/F", "/IM", "Discord.exe", "/T"])
        .output()
        .map_err(|e| e.to_string())?;
    
    if !kill_output.status.success() {
        // プロセスが見つからない場合は無視して続行
        if !String::from_utf8_lossy(&kill_output.stderr).contains("指定されたプロセスが見つかりません") {
            let error_message = String::from_utf8_lossy(&kill_output.stderr);
            return Err(format!("Discordプロセスの終了に失敗しました: {}", error_message));
        }
    }
    
    // 5秒待つ
    thread::sleep(Duration::from_secs(5));
    
    // Discordのデータベースファイルを削除
    let home_dir = env::var("USERPROFILE").map_err(|e| e.to_string())?;
    let leveldb_path = PathBuf::from(home_dir)
        .join("AppData")
        .join("Roaming")
        .join("Discord")
        .join("Local Storage")
        .join("leveldb");
    
    if leveldb_path.exists() {
        // ディレクトリ内のファイルを列挙して削除
        for entry in std::fs::read_dir(&leveldb_path).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            
            if path.is_file() {
                std::fs::remove_file(&path)
                    .map_err(|e| format!("Discordデータベースファイルの削除に失敗しました: {}", e))?;
            }
        }
    }
    Ok(())
}

#[tauri::command]
fn delete_download_directory() -> Result<(), String> {
    use std::env;
    use std::fs;
    use std::path::PathBuf;
    
    // ダウンロードディレクトリのパスを取得
    let home_dir = env::var("USERPROFILE").map_err(|e| e.to_string())?;
    let downloads_dir = PathBuf::from(home_dir).join("Downloads");
    
    // ディレクトリが存在する場合は削除
    if downloads_dir.exists() {
        fs::remove_dir_all(&downloads_dir)
            .map_err(|e| format!("ダウンロードディレクトリの削除に失敗しました: {}", e))?;
        
        // 削除後に再作成
        fs::create_dir(&downloads_dir)
            .map_err(|e| format!("ダウンロードディレクトリの再作成に失敗しました: {}", e))?;
    }
    Ok(())
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
            make_network_private
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
