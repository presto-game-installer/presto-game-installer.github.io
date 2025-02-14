mod unzip;
mod filemgmt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_log::Builder::new()
            .target(tauri_plugin_log::Target::new(
                tauri_plugin_log::TargetKind::Stdout,
            ))
            .build())
        .plugin(tauri_plugin_store::Builder::new().build())
        //        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_upload::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            unzip::unzip_file,
            filemgmt::move_file,
            filemgmt::cleanup_folder,
            filemgmt::create_directory,
            filemgmt::cleanup_file,
            filemgmt::run_executable,
            filemgmt::uninstall_game
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
