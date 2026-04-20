use tauri::command;

#[command]
pub async fn load_settings() -> Result<(), String> {
    // TODO: Implement zaroxi_infra_settings loading
    Ok(())
}

#[command]
pub async fn save_settings() -> Result<(), String> {
    // TODO: Implement zaroxi_infra_settings saving
    Ok(())
}
