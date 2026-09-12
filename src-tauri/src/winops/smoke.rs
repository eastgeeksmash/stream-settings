use windows::core::w;
use windows::Win32::Foundation::{ERROR_MORE_DATA, ERROR_SUCCESS};
use windows::Win32::Networking::NetworkListManager::{
    INetwork, INetworkListManager, NetworkListManager, NLM_ENUM_NETWORK_ALL,
};
use windows::Win32::System::Com::{CoCreateInstance, CLSCTX_ALL};
use windows::Win32::System::Power::PowerReadFriendlyName;
use windows::Win32::System::Registry::{
    RegCloseKey, RegOpenKeyExW, HKEY, HKEY_CURRENT_USER, KEY_READ,
};
use windows::Win32::UI::Shell::{FOLDERID_Downloads, SHGetKnownFolderPath, KF_FLAG_DEFAULT};

use super::com::ComInitializer;
use super::power::HIGH_PERFORMANCE_SCHEME;
use super::process::snapshot_processes;

#[test]
fn high_performance_scheme_can_be_queried() {
    let mut size = 0u32;
    let status = unsafe {
        PowerReadFriendlyName(
            None,
            Some(&HIGH_PERFORMANCE_SCHEME),
            None,
            None,
            None,
            &mut size,
        )
    };
    assert!(
        status == ERROR_SUCCESS || status == ERROR_MORE_DATA,
        "高パフォーマンス電源プランの照会に失敗しました: {status:?}"
    );
}

#[test]
fn downloads_known_folder_is_resolvable() {
    let _com = ComInitializer::new("smoke").expect("COM を初期化できませんでした");
    let pwstr = unsafe { SHGetKnownFolderPath(&FOLDERID_Downloads, KF_FLAG_DEFAULT, None) }
        .expect("Downloads フォルダを取得できませんでした");
    let path = unsafe { pwstr.to_string() }.expect("パスを文字列化できませんでした");
    unsafe { windows::Win32::System::Com::CoTaskMemFree(Some(pwstr.as_ptr().cast())) };
    assert!(!path.is_empty());
}

#[test]
fn notification_registry_keys_are_readable_or_absent() {
    for subkey in [
        w!(r"Software\Policies\Microsoft\Windows\Explorer"),
        w!(r"Software\Microsoft\Windows\CurrentVersion\PushNotifications"),
    ] {
        let mut key = HKEY::default();
        let status =
            unsafe { RegOpenKeyExW(HKEY_CURRENT_USER, subkey, Some(0), KEY_READ, &mut key) };
        if status == ERROR_SUCCESS {
            unsafe {
                let _ = RegCloseKey(key);
            }
        }
    }
}

#[test]
fn network_profiles_can_be_enumerated() {
    let _com = ComInitializer::new("smoke").expect("COM を初期化できませんでした");
    let manager: INetworkListManager =
        unsafe { CoCreateInstance(&NetworkListManager, None, CLSCTX_ALL) }
            .expect("Network List Manager を作成できませんでした");
    let enumerator = unsafe { manager.GetNetworks(NLM_ENUM_NETWORK_ALL) }
        .expect("ネットワークを列挙できませんでした");
    let mut item: [Option<INetwork>; 1] = [None];
    let mut fetched = 0u32;
    let _ = unsafe { enumerator.Next(&mut item, Some(&mut fetched)) };
}

#[test]
fn process_snapshot_is_available() {
    snapshot_processes().expect("プロセス一覧を取得できませんでした");
}
