use super::dhcp::ethernet_adapter_ids;
use super::power::disable_bus_power_saving;
use super::registry::{
    enum_hklm_subkeys, get_hklm_sz, set_existing_hklm_number, set_hkcu_dword, set_hkcu_sz,
    set_hklm_dword,
};

const NIC_CLASS: &str = r"SYSTEM\CurrentControlSet\Control\Class\{4d36e972-e325-11ce-bfc1-08002be10318}";
const NDI_OPERATION: &str = "NDI/OMT向け最適化に失敗しました";
const VMIX_OPERATION: &str = "vMix向け最適化に失敗しました";
const PNP_NO_POWER_MANAGEMENT: u32 = 0x18;
const FLOW_CONTROL_RX_TX: u32 = 3;

const DISABLE_KEYWORDS: &[&str] = &[
    "*EEE",
    "EEE",
    "*EEELinkAdvertisement",
    "EEELinkAdvertisement",
    "*EnergyEfficientEthernet",
    "EnergyEfficientEthernet",
    "*GreenEthernet",
    "GreenEthernet",
    "*InterruptModeration",
    "InterruptModeration",
    "ITR",
    "ReduceSpeedOnPowerDown",
    "*ReduceSpeedOnPowerDown",
    "ULPMode",
    "*ULPMode",
    "EnablePME",
    "*EnablePME",
    "*PriorityVLANTag",
    "PriorityVLANTag",
];

pub fn optimize_ndi_settings() -> Result<(), String> {
    tune_ethernet_adapters(NDI_OPERATION)?;
    disable_qos_reservation(NDI_OPERATION)?;
    disable_bus_power_saving()
}

pub fn optimize_vmix_settings() -> Result<(), String> {
    tune_ethernet_adapters(VMIX_OPERATION)?;
    disable_qos_reservation(VMIX_OPERATION)?;
    disable_media_throttling(VMIX_OPERATION)?;
    disable_game_overlays(VMIX_OPERATION)?;
    prefer_high_performance_for_vmix(VMIX_OPERATION)
}

fn tune_ethernet_adapters(operation: &str) -> Result<(), String> {
    let adapter_ids = ethernet_adapter_ids()?;
    let instances = enum_hklm_subkeys(NIC_CLASS, operation)?;
    for instance in instances {
        if instance.len() != 4 || !instance.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }
        let key = format!(r"{NIC_CLASS}\{instance}");
        let Some(net_cfg_id) = get_hklm_sz(&key, "NetCfgInstanceId", operation)? else {
            continue;
        };
        if !adapter_ids
            .iter()
            .any(|id| ids_match(id, &net_cfg_id))
        {
            continue;
        }

        set_hklm_dword(&key, "PnPCapabilities", PNP_NO_POWER_MANAGEMENT, operation)?;
        for keyword in DISABLE_KEYWORDS {
            let _ = set_existing_hklm_number(&key, keyword, 0, operation)?;
        }
        let _ = set_existing_hklm_number(&key, "*FlowControl", FLOW_CONTROL_RX_TX, operation)?;
        let _ = set_existing_hklm_number(&key, "FlowControl", FLOW_CONTROL_RX_TX, operation)?;
    }
    Ok(())
}

fn disable_qos_reservation(operation: &str) -> Result<(), String> {
    set_hklm_dword(
        r"SOFTWARE\Policies\Microsoft\Windows\Psched",
        "NonBestEffortLimit",
        0,
        operation,
    )
}

fn disable_media_throttling(operation: &str) -> Result<(), String> {
    set_hklm_dword(
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile",
        "NetworkThrottlingIndex",
        u32::MAX,
        operation,
    )?;
    set_hklm_dword(
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile",
        "SystemResponsiveness",
        0,
        operation,
    )
}

fn disable_game_overlays(operation: &str) -> Result<(), String> {
    set_hkcu_dword(
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\GameDVR",
        "AppCaptureEnabled",
        0,
        operation,
    )?;
    set_hkcu_dword(r"System\GameConfigStore", "GameDVR_Enabled", 0, operation)?;
    set_hkcu_dword(
        r"Software\Microsoft\GameBar",
        "AutoGameModeEnabled",
        0,
        operation,
    )?;
    set_hklm_dword(
        r"SOFTWARE\Policies\Microsoft\Windows\GameDVR",
        "AllowGameDVR",
        0,
        operation,
    )
}

fn prefer_high_performance_for_vmix(operation: &str) -> Result<(), String> {
    for path in vmix_exe_candidates() {
        if !std::path::Path::new(&path).is_file() {
            continue;
        }
        set_hkcu_sz(
            r"Software\Microsoft\DirectX\UserGpuPreferences",
            &path,
            "GpuPreference=2;",
            operation,
        )?;
        set_hkcu_sz(
            r"Software\Microsoft\Windows NT\CurrentVersion\AppCompatFlags\Layers",
            &path,
            "~ DISABLEDXMAXIMIZEDWINDOWEDMODE",
            operation,
        )?;
    }
    Ok(())
}

fn vmix_exe_candidates() -> Vec<String> {
    let mut paths = Vec::new();
    for var in ["ProgramFiles", "ProgramFiles(x86)"] {
        if let Ok(root) = std::env::var(var) {
            paths.push(format!(r"{root}\vMix\vMix64.exe"));
            paths.push(format!(r"{root}\vMix\vMix.exe"));
        }
    }
    paths
}

fn ids_match(left: &str, right: &str) -> bool {
    left.trim_matches(|c| c == '{' || c == '}')
        .eq_ignore_ascii_case(right.trim_matches(|c| c == '{' || c == '}'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adapter_ids_match_with_or_without_braces() {
        assert!(ids_match("{ABC-DEF}", "abc-def"));
        assert!(!ids_match("{ABC-DEF}", "{123}"));
    }

    #[test]
    fn ndi_keywords_cover_qos_and_power_saving() {
        assert!(DISABLE_KEYWORDS.contains(&"*EEE"));
        assert!(DISABLE_KEYWORDS.contains(&"*InterruptModeration"));
        assert!(DISABLE_KEYWORDS.contains(&"*PriorityVLANTag"));
        assert_eq!(PNP_NO_POWER_MANAGEMENT, 24);
        assert_eq!(FLOW_CONTROL_RX_TX, 3);
    }
}
