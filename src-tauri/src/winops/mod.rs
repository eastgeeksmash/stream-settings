mod admin;
mod com;
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
mod sticky_keys;
mod windows_update;

#[cfg(test)]
mod smoke;

pub use admin::{is_elevated, relaunch_as_admin};
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
pub use sticky_keys::disable_sticky_keys;
pub use windows_update::{defer_windows_update, disable_delivery_optimization};
