use eframe::egui;

pub struct Sidebar {
    pub open_workspace_dialog: bool,
    pub create_file_dialog: bool,
    pub delete_file_dialog: bool,
    pub open_file_dialog: bool,
    pub workspace_path_input: String,
    pub create_file_path_input: String,
    pub delete_file_path_input: String,
    pub open_file_path_input: String,
}

impl Default for Sidebar {
    fn default() -> Self {
        Self {
            open_workspace_dialog: false,
            create_file_dialog: false,
            delete_file_dialog: false,
            open_file_dialog: false,
            workspace_path_input: String::new(),
            create_file_path_input: String::new(),
            delete_file_path_input: String::new(),
            open_file_path_input: String::new(),
        }
    }
}

impl Sidebar {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("Workspace");
            
            if ui.button("📂 Open Workspace").clicked() {
                self.open_workspace_dialog = true;
            }
            
            ui.separator();
            
            ui.heading("File Operations");
            
            if ui.button("📄 Open File").clicked() {
                self.open_file_dialog = true;
            }
            
            if ui.button("➕ Create File").clicked() {
                self.create_file_dialog = true;
            }
            
            if ui.button("🗑️ Delete File").clicked() {
                self.delete_file_dialog = true;
            }
        });
    }
}
