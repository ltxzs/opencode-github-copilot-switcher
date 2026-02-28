pub mod error;
pub mod models;
pub mod database;
pub mod github_oauth;
pub mod auth_config;
pub mod provider_service;
pub mod commands;
pub mod state;

use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            commands::list_providers,
            commands::start_device_flow,
            commands::complete_device_flow,
            commands::delete_provider,
            commands::switch_provider,
            commands::open_url,
            commands::sync_active_account
        ])
        .setup(|_app| {
            // Setup logic will be initialized in commands/state when called
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
