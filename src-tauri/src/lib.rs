mod commands;
mod db;
mod entities;
mod models;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let state = db::setup_database(app)?;
            app.manage(state);

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::create_entity,
            commands::update_entity,
            commands::archive_entity,
            commands::list_entities,
            commands::list_tags,
            commands::search_entities,
            commands::dashboard_summary
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
