use serde::Serialize;
use windows::core::w;
use windows::Win32::Foundation::{
    ERROR_BUFFER_OVERFLOW, ERROR_INSUFFICIENT_BUFFER, ERROR_SUCCESS, WIN32_ERROR,
};
use windows::Win32::NetworkManagement::IpHelper::{
    GetAdaptersAddresses, GetInterfaceInfo, IpRenewAddress, GET_ADAPTERS_ADDRESSES_FLAGS,
    IP_ADAPTER_ADDRESSES_LH, IP_ADAPTER_INDEX_MAP, IP_INTERFACE_INFO,
};
use windows::Win32::NetworkManagement::Ndis::IfOperStatusUp;
use windows::Win32::Networking::WinSock::AF_INET;
use windows::Win32::System::Services::{
    CloseServiceHandle, OpenSCManagerW, OpenServiceW, QueryServiceStatus, StartServiceW,
    SC_MANAGER_CONNECT, SERVICE_QUERY_STATUS, SERVICE_RUNNING, SERVICE_START,
    SERVICE_START_PENDING,
};

use super::com::ComInitializer;
use super::error::{from_win32_error, from_windows_error, is_class_not_registered};
use super::registry::{delete_hklm_value, set_hklm_dword};

const OPERATION: &str = "DHCP設定の変更に失敗しました";
const IF_TYPE_SOFTWARE_LOOPBACK: u32 = 24;
const IF_TYPE_ETHERNET_CSMACD: u32 = 6;

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct NetworkAdapterInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub dhcp_enabled: bool,
    pub connected: bool,
}

#[link(name = "dhcpcsvc")]
unsafe extern "system" {
    fn DhcpNotifyConfigChange(
        server_name: *const u16,
        adapter_name: *const u16,
        is_new_ip: i32,
        ip_index: u32,
        ip_address: u32,
        subnet_mask: u32,
        dhcp_action: i32,
    ) -> u32;
}

pub fn list_network_adapters() -> Result<Vec<NetworkAdapterInfo>, String> {
    Ok(collect_adapters()?
        .into_iter()
        .map(|adapter| adapter.info)
        .collect())
}

pub fn enable_dhcp(adapter_id: String) -> Result<(), String> {
    let adapters = collect_adapters()?;
    let adapter = adapters
        .into_iter()
        .find(|adapter| adapter.info.id.eq_ignore_ascii_case(&adapter_id))
        .ok_or_else(|| format!("{OPERATION}: 指定したネットワークアダプターが見つかりません。"))?;

    enable_dhcp_for_adapter(&adapter)
}

pub fn enable_dhcp_for_connected_adapters() -> Result<(), String> {
    let adapters = collect_adapters()?;
    let targets: Vec<AdapterRecord> = adapters
        .into_iter()
        .filter(|adapter| adapter.info.connected && !adapter.info.dhcp_enabled)
        .collect();

    if targets.is_empty() {
        return Ok(());
    }

    let mut failures = Vec::new();
    for adapter in targets {
        if let Err(error) = enable_dhcp_for_adapter(&adapter) {
            failures.push(error);
        }
    }

    if let Some(error) = failures.into_iter().next() {
        return Err(error);
    }

    Ok(())
}

struct AdapterRecord {
    info: NetworkAdapterInfo,
    if_index: u32,
    if_type: u32,
}

pub(super) fn ethernet_adapter_ids() -> Result<Vec<String>, String> {
    Ok(collect_adapters()?
        .into_iter()
        .filter(|adapter| adapter.if_type == IF_TYPE_ETHERNET_CSMACD)
        .map(|adapter| adapter.info.id)
        .collect())
}

fn enable_dhcp_for_adapter(adapter: &AdapterRecord) -> Result<(), String> {
    let _com = ComInitializer::new(OPERATION)?;
    let _ = ensure_dhcp_client_running();

    let interface_key = format!(
        r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces\{}",
        adapter.info.id.trim_matches(|c| c == '{' || c == '}')
    );
    // AdapterName from GetAdaptersAddresses already includes braces. Try both.
    let key_with_braces = format!(
        r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces\{}",
        adapter.info.id
    );

    let registry_key = if adapter.info.id.starts_with('{') {
        key_with_braces
    } else {
        interface_key
    };

    set_hklm_dword(&registry_key, "EnableDHCP", 1, OPERATION)?;
    delete_hklm_value(&registry_key, "IPAddress", OPERATION)?;
    delete_hklm_value(&registry_key, "SubnetMask", OPERATION)?;
    delete_hklm_value(&registry_key, "DefaultGateway", OPERATION)?;

    let notify_status = notify_dhcp_enabled(&adapter.info.id);
    if notify_status != 0 && !is_class_not_registered(notify_status) {
        return Err(from_win32_error(OPERATION, WIN32_ERROR(notify_status)));
    }

    let mut map = interface_map_for_adapter(adapter)?;
    let renew = unsafe { IpRenewAddress(&mut map) };
    if renew != ERROR_SUCCESS.0 && notify_status != 0 {
        return Err(from_win32_error(OPERATION, WIN32_ERROR(renew)));
    }

    Ok(())
}

fn notify_dhcp_enabled(adapter_id: &str) -> u32 {
    for name in adapter_name_candidates(adapter_id) {
        let adapter_name: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
        let status =
            unsafe { DhcpNotifyConfigChange(std::ptr::null(), adapter_name.as_ptr(), 0, 0, 0, 0, 1) };
        if status == 0 || !is_class_not_registered(status) {
            return status;
        }
    }
    0x8004_0154
}

fn adapter_name_candidates(adapter_id: &str) -> Vec<String> {
    let trimmed = adapter_id.trim_matches(|c| c == '{' || c == '}').to_string();
    let braced = format!("{{{trimmed}}}");
    vec![adapter_id.to_string(), braced, trimmed]
}

fn interface_map_for_adapter(adapter: &AdapterRecord) -> Result<IP_ADAPTER_INDEX_MAP, String> {
    if let Some(map) = interface_map_from_system(adapter.if_index)? {
        return Ok(map);
    }

    let mut map = IP_ADAPTER_INDEX_MAP {
        Index: adapter.if_index,
        Name: [0; 128],
    };
    let device_name = format!(
        r"\DEVICE\TCPIP_{}",
        if adapter.info.id.starts_with('{') {
            adapter.info.id.clone()
        } else {
            format!("{{{}}}", adapter.info.id)
        }
    );
    let name_utf16: Vec<u16> = device_name.encode_utf16().collect();
    let copy_len = name_utf16.len().min(map.Name.len().saturating_sub(1));
    map.Name[..copy_len].copy_from_slice(&name_utf16[..copy_len]);
    Ok(map)
}

fn interface_map_from_system(if_index: u32) -> Result<Option<IP_ADAPTER_INDEX_MAP>, String> {
    let mut size = 0u32;
    let first = unsafe { GetInterfaceInfo(None, &mut size) };
    if first != ERROR_INSUFFICIENT_BUFFER.0 && first != ERROR_SUCCESS.0 {
        return Ok(None);
    }

    let mut buffer = vec![0u8; size.max(std::mem::size_of::<IP_INTERFACE_INFO>() as u32) as usize];
    let info = buffer.as_mut_ptr().cast::<IP_INTERFACE_INFO>();
    let status = unsafe { GetInterfaceInfo(Some(&mut *info), &mut size) };
    if status != ERROR_SUCCESS.0 {
        return Ok(None);
    }

    let count = unsafe { (*info).NumAdapters }.max(0) as usize;
    let adapters = unsafe { std::slice::from_raw_parts((*info).Adapter.as_ptr(), count) };
    Ok(adapters
        .iter()
        .find(|map| map.Index == if_index)
        .cloned())
}

fn ensure_dhcp_client_running() -> Result<(), String> {
    let scm = unsafe { OpenSCManagerW(None, None, SC_MANAGER_CONNECT) }
        .map_err(|error| from_windows_error(OPERATION, error))?;
    let service = unsafe { OpenServiceW(scm, w!("Dhcp"), SERVICE_QUERY_STATUS | SERVICE_START) };
    let service = match service {
        Ok(service) => service,
        Err(_) => {
            unsafe {
                let _ = CloseServiceHandle(scm);
            }
            return Ok(());
        }
    };

    let mut status = Default::default();
    let queried = unsafe { QueryServiceStatus(service, &mut status) };
    if queried.is_ok()
        && status.dwCurrentState != SERVICE_RUNNING
        && status.dwCurrentState != SERVICE_START_PENDING
    {
        let _ = unsafe { StartServiceW(service, None) };
    }

    unsafe {
        let _ = CloseServiceHandle(service);
        let _ = CloseServiceHandle(scm);
    }
    Ok(())
}

fn collect_adapters() -> Result<Vec<AdapterRecord>, String> {
    let mut size = 0u32;
    let flags = GET_ADAPTERS_ADDRESSES_FLAGS(0);
    let first = unsafe { GetAdaptersAddresses(AF_INET.0 as u32, flags, None, None, &mut size) };
    if first != ERROR_BUFFER_OVERFLOW.0 && first != ERROR_SUCCESS.0 {
        return Err(from_win32_error(
            OPERATION,
            windows::Win32::Foundation::WIN32_ERROR(first),
        ));
    }

    let mut buffer = vec![0u8; size as usize];
    let head = buffer.as_mut_ptr().cast::<IP_ADAPTER_ADDRESSES_LH>();
    let status =
        unsafe { GetAdaptersAddresses(AF_INET.0 as u32, flags, None, Some(head), &mut size) };
    if status != ERROR_SUCCESS.0 {
        return Err(from_win32_error(
            OPERATION,
            windows::Win32::Foundation::WIN32_ERROR(status),
        ));
    }

    let mut adapters = Vec::new();
    let mut current = head;
    while !current.is_null() {
        let adapter = unsafe { &*current };
        if adapter.IfType != IF_TYPE_SOFTWARE_LOOPBACK {
            let id = unsafe { adapter.AdapterName.to_string() }.unwrap_or_default();
            let name = unsafe { adapter.FriendlyName.to_string() }.unwrap_or_default();
            let description = unsafe { adapter.Description.to_string() }.unwrap_or_default();
            if !id.is_empty() && !name.is_empty() {
                let (dhcp_enabled, if_index) = unsafe {
                    (
                        (adapter.Anonymous2.Flags & (1 << 2)) != 0,
                        adapter.Anonymous1.Anonymous.IfIndex,
                    )
                };
                adapters.push(AdapterRecord {
                    info: NetworkAdapterInfo {
                        id,
                        name,
                        description,
                        dhcp_enabled,
                        connected: adapter.OperStatus == IfOperStatusUp,
                    },
                    if_index,
                    if_type: adapter.IfType,
                });
            }
        }
        current = adapter.Next;
    }

    Ok(adapters)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adapter_info_keeps_id_and_name() {
        let adapter = NetworkAdapterInfo {
            id: "{abc}".into(),
            name: "Ethernet".into(),
            description: "USB LAN".into(),
            dhcp_enabled: false,
            connected: true,
        };
        assert_eq!(adapter.name, "Ethernet");
        assert!(!adapter.dhcp_enabled);
    }

    #[test]
    fn dhcp_notify_accepts_guid_with_and_without_braces() {
        let names = adapter_name_candidates("{abc-def}");
        assert!(names.iter().any(|name| name == "{abc-def}"));
        assert!(names.iter().any(|name| name == "abc-def"));
    }
}
