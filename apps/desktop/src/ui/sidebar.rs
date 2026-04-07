use eframe::egui;
use crate::events::SidebarEvent;

pub struct Sidebar {
    events: Vec<SidebarEvent>,
}

impl Default for Sidebar {
    fn default() -> Self {
        Self {
            events: Vec::new(),
        }
    }
}

impl Sidebar {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        // Clear previous events
        self.events.clear();
        
        ui.vertical(|ui| {
            ui.heading("Workspace");
            
            if ui.button("📂 Open Workspace").clicked() {
                self.events.push(SidebarEvent::OpenWorkspace);
            }
            
            ui.separator();
            
            ui.heading("File Operations");
            
            if ui.button("📄 Open File").clicked() {
                self.events.push(SidebarEvent::OpenFile);
            }
            
            if ui.button("➕ Create File").clicked() {
                self.events.push(SidebarEvent::CreateFile);
            }
            
            if ui.button("🗑️ Delete File").clicked() {
                self.events.push(SidebarEvent::DeleteFile);
            }
        });
    }

    pub fn take_events(&mut self) -> Vec<SidebarEvent> {
        std::mem::take(&mut self.events)
    }
}
