use tauri::{Window, State, command, Runtime};
use crate::PenetrableState;
use crate::com;

#[command]
pub fn enable_penetrable<R: Runtime>(window: Window<R>, state: State<'_, PenetrableState>) {
    com::windows::penetrable(window);
    let mut enabled = state.enabled.lock().unwrap();
    *enabled = true;
}

#[command]
pub fn disable_penetrable<R: Runtime>(window: Window<R>, state: State<'_, PenetrableState>) {
    com::windows::disable_penetrable(window);
    let mut enabled = state.enabled.lock().unwrap();
    *enabled = false;
}