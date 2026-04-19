use tauri::{App, AppHandle};

pub fn init_app(_app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize your app here
    Ok(())
}

pub fn on_app_ready(_app_handle: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // App is ready, start services
    Ok(())
}
