fn main() {
    let level = if tauri_build::is_dev() {
        "asInvoker"
    } else {
        "requireAdministrator"
    };
    let manifest = include_str!("windows/app.manifest").replace("__EXECUTION_LEVEL__", level);
    let windows = tauri_build::WindowsAttributes::new().app_manifest(manifest);
    tauri_build::try_build(tauri_build::Attributes::new().windows_attributes(windows))
        .expect("failed to run build script");
}
