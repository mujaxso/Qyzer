use tauri::command;

#[command]
pub async fn load_settings() -> Result<(), String> {
    // TODO: Implement settings loading
    Ok(())
}

#[command]
pub async fn save_settings() -> Result<(), String> {
    // TODO: Implement settings saving
    Ok(())
}
