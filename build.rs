const COMMANDS: &[&str] = &["enable_penetrable", "disable_penetrable"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .build();
}