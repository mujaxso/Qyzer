use tauri::command;

#[command]
pub async fn get_document() -> Result<(), String> {
    // TODO: Implement document retrieval
    Ok(())
}

#[command]
pub async fn apply_edit() -> Result<(), String> {
    // TODO: Implement edit application
    Ok(())
}

#[command]
pub async fn save_document() -> Result<(), String> {
    // TODO: Implement document saving
    Ok(())
}
