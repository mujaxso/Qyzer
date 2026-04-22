use tauri::AppHandle;
use crate::services::theme_service::ThemeService;

pub fn on_app_ready(app_handle: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // App is ready, start services
    let theme_service = ThemeService::new(app_handle.clone());
    
    // Apply theme on startup with a small delay to ensure frontend is ready
    tauri::async_runtime::spawn(async move {
        // Wait for frontend to be fully loaded
        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        theme_service.apply_theme().await;
    });
    
    Ok(())
}
