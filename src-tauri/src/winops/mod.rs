mod admin;
mod com;
mod debloat;
mod desktop;
mod dhcp;
mod error;
mod folders;
mod network;
mod nvidia;
mod onedrive;
mod optimize;
mod power;
mod process;
mod purge;
mod registry;
mod shell;
mod sound;
mod sticky_keys;
mod windows_update;
mod winget;

#[cfg(test)]
mod smoke;

pub use admin::{is_elevated, relaunch_as_admin};
pub use debloat::{
    disable_bing_search, disable_copilot, disable_fast_startup, disable_recall_and_click_to_do,
    disable_storage_sense, disable_suggestions, disable_telemetry, disable_widgets,
    remove_debloat_group,
};
pub use desktop::{
    disable_aero, hide_desktop_icons_and_taskbar, restore_aero, restore_desktop_icons_and_taskbar,
    set_solid_wallpaper,
};
pub use dhcp::{
    enable_dhcp, enable_dhcp_for_connected_adapters, list_network_adapters, NetworkAdapterInfo,
};
pub use folders::delete_download_directory;
pub use network::make_network_private;
pub use nvidia::optimize_nvidia_settings;
pub use onedrive::disable_onedrive_sync;
pub use optimize::{optimize_ndi_settings, optimize_vmix_settings};
pub use power::change_power_settings;
pub use process::logout_discord;
pub use purge::{purge_obs_settings, purge_vmix_settings, reset_vmix_registration};
pub use registry::disable_windows_notifications;
pub use shell::logout_chrome;
pub use sound::{disable_call_ducking, set_no_sounds_scheme};
pub use sticky_keys::disable_sticky_keys;
pub use windows_update::{defer_windows_update, disable_delivery_optimization};
pub use winget::{install_winget_package, resolve_winget};
