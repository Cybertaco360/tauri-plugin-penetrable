use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
    Manager,
    Config
};
use std::sync::Mutex;
mod com;
mod commands;

pub struct PenetrableState {
    pub enabled: Mutex<bool>,
}

pub fn init<R: Runtime>() -> TauriPlugin<R, Config> {
    Builder::<R, Config>::new("penetrable")
        .invoke_handler(tauri::generate_handler![
            commands::enable_penetrable,
            commands::disable_penetrable
        ])
        .setup(|app, _api| {
            app.manage(PenetrableState {
                enabled: Mutex::new(false),
            });
            Ok(())
        })
        .build()
}