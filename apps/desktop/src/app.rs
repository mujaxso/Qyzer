// Re-export the main types from the crate root modules
pub use crate::message::Message;
pub use crate::state::App;
pub use crate::update::update;
pub use crate::view::view;

use iced::{Element, Command};

impl iced::Application for App {
    type Message = Message;
    type Theme = iced::Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let (app, command) = App::new();
        
        // Load custom fonts for icon support
        // We'll try to load multiple fonts to ensure icons are visible
        let mut font_commands = Vec::new();
        
        // Try to load fonts from various possible locations
        // The binary might be running from different directories
        let possible_font_dirs = [
            // When running from project root
            "apps/desktop/assets/fonts",
            // When running from desktop directory
            "assets/fonts",
            // When running from target directory
            "../assets/fonts",
        ];
        
        let mut font_paths = Vec::new();
        
        for dir in &possible_font_dirs {
            let sans_path = format!("{}/NotoSans-Regular.ttf", dir);
            let emoji_path = format!("{}/NotoColorEmoji.ttf", dir);
            let emoji_alt_path = format!("{}/NotoEmoji-Regular.ttf", dir);
            
            if std::path::Path::new(&sans_path).exists() {
                font_paths.push((sans_path, "Noto Sans"));
            }
            if std::path::Path::new(&emoji_path).exists() {
                font_paths.push((emoji_path, "Noto Color Emoji"));
            } else if std::path::Path::new(&emoji_alt_path).exists() {
                font_paths.push((emoji_alt_path, "Noto Emoji"));
            }
        }
        
        // If no fonts found in standard locations, try current directory
        if font_paths.is_empty() {
            let current_dir_fonts = [
                "NotoSans-Regular.ttf",
                "NotoColorEmoji.ttf",
                "NotoEmoji-Regular.ttf",
            ];
            for font_file in &current_dir_fonts {
                if std::path::Path::new(font_file).exists() {
                    font_paths.push((font_file.to_string(), font_file));
                }
            }
        }
        
        for (path, name) in font_paths {
            match std::fs::read(&path) {
                Ok(bytes) => {
                    font_commands.push(
                        iced::font::load(bytes)
                            .map(|_| Message::FontLoaded)
                            .map_err(|_| Message::FontLoadFailed)
                    );
                    #[cfg(debug_assertions)]
                    eprintln!("Loaded font: {} from {}", name, path);
                }
                Err(e) => {
                    #[cfg(debug_assertions)]
                    eprintln!("Failed to load font {} from {}: {}", name, path, e);
                }
            }
        }
        
        if font_commands.is_empty() {
            #[cfg(debug_assertions)]
            eprintln!("Warning: No custom fonts loaded. Icons may not display correctly.");
            #[cfg(debug_assertions)]
            eprintln!("Run from apps/desktop directory: ./scripts/download-fonts.sh");
        }
        
        // If no fonts were loaded, we'll just use system fonts
        if font_commands.is_empty() {
            // Log a warning in debug mode
            #[cfg(debug_assertions)]
            eprintln!("No custom fonts loaded. Icons may not display correctly. Run `scripts/download-fonts.sh` to download required fonts.");
            (app, command)
        } else {
            (app, Command::batch(font_commands).then(|| command))
        }
    }

    fn title(&self) -> String {
        String::from("Neote")
    }

    fn theme(&self) -> iced::Theme {
        self.theme.to_iced_theme()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        update(self, message)
    }

    fn view(&self) -> Element<'_, Message> {
        view(self)
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        self.subscription()
    }
}
