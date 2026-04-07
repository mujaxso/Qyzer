#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum SidebarEvent {
    OpenWorkspace,
    CreateFile,
    DeleteFile,
    OpenFile,
}
