use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::{Command, Output};

use serde::Deserialize;

use windows::Win32::Foundation::{LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    SendMessageTimeoutW, HWND_BROADCAST, SMTO_ABORTIFHUNG, WM_SETTINGCHANGE,
};

use super::registry::{
    set_hkcu_dword, set_hkcu_dword_result, set_hklm_dword, set_hklm_dword_result, RegistryWriteFailure,
};

const CREATE_NO_WINDOW: u32 = 0x0800_0000;

const TELEMETRY_OPERATION: &str = "テレメトリの無効化に失敗しました";
const SUGGESTIONS_OPERATION: &str = "ヒントと提案の無効化に失敗しました";
const COPILOT_OPERATION: &str = "Copilotの無効化に失敗しました";
const RECALL_OPERATION: &str = "RecallとClick to Doの無効化に失敗しました";
const WIDGETS_OPERATION: &str = "ウィジェットの無効化に失敗しました";
const BING_OPERATION: &str = "Bing検索の無効化に失敗しました";
const FAST_STARTUP_OPERATION: &str = "高速スタートアップの無効化に失敗しました";
const STORAGE_SENSE_OPERATION: &str = "Storage Senseの無効化に失敗しました";
const APP_REMOVAL_OPERATION: &str = "不要アプリの削除に失敗しました";

pub const DEBLOAT_GROUPS: &[&str] = &["default", "gaming", "hp"];
const CATALOG_JSON: &str = include_str!("../../resources/debloat/apps.json");

enum Hive {
    Hkcu,
    Hklm,
}

struct DwordSetting {
    hive: Hive,
    subkey: &'static str,
    name: &'static str,
    value: u32,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct Catalog {
    groups: CatalogGroups,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct CatalogGroups {
    default: Vec<AppEntry>,
    gaming: Vec<AppEntry>,
    hp: Vec<AppEntry>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct AppEntry {
    id: String,
    method: String,
}

pub fn disable_telemetry() -> Result<(), String> {
    apply_settings(
        &[
            hkcu(r"Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo", "Enabled", 0),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Privacy",
                "TailoredExperiencesWithDiagnosticDataEnabled",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Speech_OneCore\Settings\OnlineSpeechPrivacy",
                "HasAccepted",
                0,
            ),
            hkcu(r"Software\Microsoft\Input\TIPC", "Enabled", 0),
            hkcu(
                r"Software\Microsoft\InputPersonalization",
                "RestrictImplicitInkCollection",
                1,
            ),
            hkcu(
                r"Software\Microsoft\InputPersonalization",
                "RestrictImplicitTextCollection",
                1,
            ),
            hkcu(
                r"Software\Microsoft\InputPersonalization\TrainedDataStore",
                "HarvestContacts",
                0,
            ),
            hkcu(r"Software\Microsoft\Personalization\Settings", "AcceptedPrivacyPolicy", 0),
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection",
                "AllowTelemetry",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                "Start_TrackProgs",
                0,
            ),
            hklm(r"SOFTWARE\Policies\Microsoft\Windows\System", "PublishUserActivities", 0),
            hkcu(r"Software\Microsoft\Siuf\Rules", "NumberOfSIUFInPeriod", 0),
            hklm(r"SOFTWARE\Policies\Microsoft\Edge", "PersonalizationReportingEnabled", 0),
            hklm(r"SOFTWARE\Policies\Microsoft\Edge", "DiagnosticData", 0),
        ],
        TELEMETRY_OPERATION,
    )
}

pub fn disable_suggestions() -> Result<(), String> {
    apply_settings(
        &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                "SubscribedContent-310093Enabled",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                "SubscribedContent-338388Enabled",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                "SystemPaneSuggestionsEnabled",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                "Start_IrisRecommendations",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                "SubscribedContent-338389Enabled",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                "SoftLandingEnabled",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                "SubscribedContent-338393Enabled",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                "SubscribedContent-353694Enabled",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                "SubscribedContent-353696Enabled",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                "SubscribedContent-353698Enabled",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\SystemSettings\AccountNotifications",
                "EnableAccountNotifications",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\UserProfileEngagement",
                "ScoobeSystemSettingEnabled",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                "ShowSyncProviderNotifications",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                "SilentInstalledAppsEnabled",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Notifications\Settings\Windows.SystemToast.Suggested",
                "Enabled",
                0,
            ),
            hkcu(r"Software\Microsoft\Windows\CurrentVersion\Mobility", "OptedIn", 0),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                "Start_AccountNotifications",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Notifications\Settings\Windows.SystemToast.BackupReminder",
                "Enabled",
                0,
            ),
        ],
        SUGGESTIONS_OPERATION,
    )
}

pub fn disable_copilot() -> Result<(), String> {
    apply_settings(
        &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                "ShowCopilotButton",
                0,
            ),
            hkcu(
                r"Software\Policies\Microsoft\Windows\WindowsCopilot",
                "TurnOffWindowsCopilot",
                1,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\WindowsCopilot",
                "TurnOffWindowsCopilot",
                1,
            ),
        ],
        COPILOT_OPERATION,
    )
}

pub fn disable_recall_and_click_to_do() -> Result<(), String> {
    apply_settings(
        &[
            hkcu(r"Software\Policies\Microsoft\Windows\WindowsAI", "DisableAIDataAnalysis", 1),
            hklm(r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI", "DisableAIDataAnalysis", 1),
            hklm(r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI", "AllowRecallEnablement", 0),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
                "TurnOffSavingSnapshots",
                1,
            ),
            hkcu(r"Software\Policies\Microsoft\Windows\WindowsAI", "DisableClickToDo", 1),
            hklm(r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI", "DisableClickToDo", 1),
        ],
        RECALL_OPERATION,
    )
}

const WIDGET_SETTINGS: &[DwordSetting] = &[
    hklm(r"SOFTWARE\Policies\Microsoft\Dsh", "DisableWidgetsBoard", 1),
    hklm(r"SOFTWARE\Policies\Microsoft\Dsh", "AllowNewsAndInterests", 0),
    hkcu(
        r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
        "TaskbarDa",
        0,
    ),
];

const BING_SETTINGS: &[DwordSetting] = &[
    hkcu(
        r"Software\Microsoft\Windows\CurrentVersion\Search",
        "BingSearchEnabled",
        0,
    ),
    hkcu(
        r"Software\Microsoft\Windows\CurrentVersion\Search",
        "CortanaConsent",
        0,
    ),
    hkcu(
        r"Software\Policies\Microsoft\Windows\Explorer",
        "DisableSearchBoxSuggestions",
        1,
    ),
    hklm(
        r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
        "AllowCortana",
        0,
    ),
    hklm(
        r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
        "CortanaConsent",
        0,
    ),
    hklm(
        r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
        "BingSearchEnabled",
        0,
    ),
    hklm(
        r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
        "ConnectedSearchUseWeb",
        0,
    ),
    hklm(
        r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
        "AllowCloudSearch",
        0,
    ),
    hklm(
        r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
        "DisableWebSearch",
        1,
    ),
];

pub fn disable_widgets() -> Result<(), String> {
    let results = apply_each(WIDGET_SETTINGS, WIDGETS_OPERATION);
    if setting_applied(&results, "DisableWidgetsBoard", r"SOFTWARE\Policies\Microsoft\Dsh") {
        notify_policy_changed();
        return Ok(());
    }
    Err(first_failure_message(&results, WIDGETS_OPERATION))
}

pub fn disable_bing_search() -> Result<(), String> {
    let results = apply_each(BING_SETTINGS, BING_OPERATION);
    if setting_applied(
        &results,
        "BingSearchEnabled",
        r"Software\Microsoft\Windows\CurrentVersion\Search",
    ) {
        notify_policy_changed();
        return Ok(());
    }
    Err(first_failure_message(&results, BING_OPERATION))
}

pub fn disable_fast_startup() -> Result<(), String> {
    apply_settings(
        &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\Power",
            "HiberbootEnabled",
            0,
        )],
        FAST_STARTUP_OPERATION,
    )
}

pub fn disable_storage_sense() -> Result<(), String> {
    apply_settings(
        &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\StorageSense\Parameters\StoragePolicy",
            "01",
            0,
        )],
        STORAGE_SENSE_OPERATION,
    )
}

pub fn remove_debloat_group(dir: &Path, group: &str, winget: Option<&Path>) -> Result<(), String> {
    if !DEBLOAT_GROUPS.contains(&group) {
        return Err(format!("{APP_REMOVAL_OPERATION}: 対象外のグループです。"));
    }

    let script = dir.join("remove-apps.ps1");
    let catalog = dir.join("apps.json");
    if !script.is_file() || !catalog.is_file() {
        return Err(format!("{APP_REMOVAL_OPERATION}: 削除スクリプトが見つかりません。"));
    }

    let mut command = Command::new("powershell.exe");
    command
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            &script.to_string_lossy(),
            "-Catalog",
            &catalog.to_string_lossy(),
            "-Group",
            group,
        ])
        .creation_flags(CREATE_NO_WINDOW);
    if let Some(winget) = winget {
        command.args(["-WingetPath", &winget.to_string_lossy()]);
    }

    let output = command
        .output()
        .map_err(|error| format!("{APP_REMOVAL_OPERATION}: {error}"))?;
    if output.status.success() {
        return Ok(());
    }
    Err(format!("{APP_REMOVAL_OPERATION}: {}", output_detail(&output)))
}

struct SettingOutcome {
    subkey: &'static str,
    name: &'static str,
    result: Result<(), RegistryWriteFailure>,
}

fn apply_settings(settings: &[DwordSetting], operation: &str) -> Result<(), String> {
    for setting in settings {
        match setting.hive {
            Hive::Hkcu => set_hkcu_dword(setting.subkey, setting.name, setting.value, operation)?,
            Hive::Hklm => set_hklm_dword(setting.subkey, setting.name, setting.value, operation)?,
        }
    }
    Ok(())
}

fn apply_each(settings: &[DwordSetting], operation: &str) -> Vec<SettingOutcome> {
    settings
        .iter()
        .map(|setting| SettingOutcome {
            subkey: setting.subkey,
            name: setting.name,
            result: match setting.hive {
                Hive::Hkcu => {
                    set_hkcu_dword_result(setting.subkey, setting.name, setting.value, operation)
                }
                Hive::Hklm => {
                    set_hklm_dword_result(setting.subkey, setting.name, setting.value, operation)
                }
            },
        })
        .collect()
}

fn setting_applied(results: &[SettingOutcome], name: &str, subkey: &str) -> bool {
    results
        .iter()
        .any(|result| result.name == name && result.subkey == subkey && result.result.is_ok())
}

fn first_failure_message(results: &[SettingOutcome], operation: &str) -> String {
    results
        .iter()
        .find_map(|result| result.result.as_ref().err().map(|failure| failure.message.clone()))
        .unwrap_or_else(|| format!("{operation}: 設定を書き込めませんでした。"))
}

fn notify_policy_changed() {
    let parameter: Vec<u16> = "Policy".encode_utf16().chain(std::iter::once(0)).collect();
    unsafe {
        let _ = SendMessageTimeoutW(
            HWND_BROADCAST,
            WM_SETTINGCHANGE,
            WPARAM(0),
            LPARAM(parameter.as_ptr() as isize),
            SMTO_ABORTIFHUNG,
            1000,
            None,
        );
    }
}

const fn hkcu(subkey: &'static str, name: &'static str, value: u32) -> DwordSetting {
    DwordSetting {
        hive: Hive::Hkcu,
        subkey,
        name,
        value,
    }
}

const fn hklm(subkey: &'static str, name: &'static str, value: u32) -> DwordSetting {
    DwordSetting {
        hive: Hive::Hklm,
        subkey,
        name,
        value,
    }
}

fn output_detail(output: &Output) -> String {
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let text = if stderr.trim().is_empty() {
        stdout.trim()
    } else {
        stderr.trim()
    };
    let lines: Vec<&str> = text.lines().rev().take(12).collect();
    let detail = lines.into_iter().rev().collect::<Vec<_>>().join("\n");
    if detail.is_empty() {
        format!("終了コード {}", output.status.code().unwrap_or(-1))
    } else {
        detail
    }
}

fn catalog() -> Catalog {
    serde_json::from_str(CATALOG_JSON).expect("debloat apps.json")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn telemetry_keeps_required_diagnostic_policy() {
        assert_eq!(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection",
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection",
                "AllowTelemetry",
                0,
            )
            .subkey
        );
    }

    #[test]
    fn fast_startup_and_storage_sense_keys() {
        assert_eq!(
            "HiberbootEnabled",
            hklm(
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Power",
                "HiberbootEnabled",
                0,
            )
            .name
        );
        assert_eq!(
            "01",
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\StorageSense\Parameters\StoragePolicy",
                "01",
                0,
            )
            .name
        );
    }

    #[test]
    fn widget_disable_keeps_board_policy_when_other_values_are_blocked() {
        assert!(WIDGET_SETTINGS.iter().any(|setting| {
            setting.name == "DisableWidgetsBoard" && setting.subkey.ends_with(r"\Dsh")
        }));
        assert!(WIDGET_SETTINGS.iter().any(|setting| setting.name == "TaskbarDa"));
    }

    #[test]
    fn bing_disable_includes_start_menu_web_search() {
        assert!(BING_SETTINGS.iter().any(|setting| {
            setting.name == "BingSearchEnabled"
                && setting.subkey == r"Software\Microsoft\Windows\CurrentVersion\Search"
                && setting.value == 0
        }));
    }

    #[test]
    fn catalog_groups_exclude_edge_and_cover_targets() {
        let catalog = catalog();
        let all = catalog
            .groups
            .default
            .iter()
            .chain(catalog.groups.gaming.iter())
            .chain(catalog.groups.hp.iter());
        assert!(all.clone().all(|app| app.method == "Appx" || app.method == "WinGet"));
        assert!(all.clone().all(|app| app.id != "Microsoft.Edge" && app.id != "XPFFTQ037JWMHS"));
        assert!(catalog.groups.default.iter().any(|app| app.id == "Microsoft.BingNews"));
        assert!(catalog.groups.default.iter().any(|app| app.id == "XP9CXNGPPJ97XX" && app.method == "WinGet"));
        assert!(catalog
            .groups
            .gaming
            .iter()
            .any(|app| app.id == "Microsoft.XboxGamingOverlay"));
        assert!(catalog
            .groups
            .hp
            .iter()
            .any(|app| app.id == "AD2F1837.HPSupportAssistant"));
        assert!(!catalog.groups.hp.is_empty());
        assert_eq!(DEBLOAT_GROUPS, ["default", "gaming", "hp"]);
    }
}
