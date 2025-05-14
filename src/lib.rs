use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};
mod com;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("penetrable")
        .invoke_handler(tauri::generate_handler![enable_penetrable, disable_penetrable])
        .on_window_ready(|window| {
            // Optional: Enable penetrable by default or leave it as an explicit action.
        })
        .build()
}

#[tauri::command]
fn enable_penetrable<R: Runtime>(window: Window<R>) {
    com::windows::penetrable(window);
}

#[tauri::command]
fn disable_penetrable<R: Runtime>(window: Window<R>) {
    com::windows::disable_penetrable(window);
}
