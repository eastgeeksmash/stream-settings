mod com;
mod error;
mod folders;
mod network;
mod power;
mod process;
mod registry;
mod shell;

#[cfg(test)]
mod smoke;

pub use folders::delete_download_directory;
pub use network::make_network_private;
pub use power::change_power_settings;
pub use process::logout_discord;
pub use registry::disable_windows_notifications;
pub use shell::logout_chrome;
