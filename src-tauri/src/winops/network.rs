use windows::Win32::Foundation::S_FALSE;
use windows::Win32::Networking::NetworkListManager::{
    INetwork, INetworkListManager, NetworkListManager, NLM_ENUM_NETWORK_ALL,
    NLM_NETWORK_CATEGORY_PRIVATE, NLM_NETWORK_CATEGORY_PUBLIC,
};
use windows::Win32::System::Com::{CoCreateInstance, CLSCTX_ALL};

use super::com::ComInitializer;
use super::error::from_windows_error;

const OPERATION: &str = "ネットワーク設定の変更に失敗しました";

pub fn make_network_private() -> Result<(), String> {
    let _com = ComInitializer::new(OPERATION)?;
    let manager: INetworkListManager = unsafe {
        CoCreateInstance(&NetworkListManager, None, CLSCTX_ALL)
            .map_err(|error| from_windows_error(OPERATION, error))?
    };

    let enumerator = unsafe {
        manager
            .GetNetworks(NLM_ENUM_NETWORK_ALL)
            .map_err(|error| from_windows_error(OPERATION, error))?
    };

    let mut failures = Vec::new();
    loop {
        let mut item: [Option<INetwork>; 1] = [None];
        let mut fetched = 0u32;
        match unsafe { enumerator.Next(&mut item, Some(&mut fetched)) } {
            Ok(()) if fetched == 0 => break,
            Ok(()) => {
                if let Some(network) = item[0].take() {
                    if let Err(error) = set_public_network_private(&network) {
                        failures.push(error);
                    }
                }
            }
            Err(error) if error.code() == S_FALSE => break,
            Err(error) => return Err(from_windows_error(OPERATION, error)),
        }
    }

    if let Some(error) = failures.into_iter().next() {
        return Err(error);
    }

    Ok(())
}

fn set_public_network_private(network: &INetwork) -> Result<(), String> {
    let category = unsafe {
        network
            .GetCategory()
            .map_err(|error| from_windows_error(OPERATION, error))?
    };

    if category != NLM_NETWORK_CATEGORY_PUBLIC {
        return Ok(());
    }

    unsafe {
        network
            .SetCategory(NLM_NETWORK_CATEGORY_PRIVATE)
            .map_err(|error| from_windows_error(OPERATION, error))
    }
}
