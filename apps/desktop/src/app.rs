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
        
        // Try to load Noto Sans if the file exists
        if let Ok(bytes) = std::fs::read("assets/fonts/NotoSans-Regular.ttf") {
            font_commands.push(
                iced::font::load(bytes)
                    .map(|_| Message::FontLoaded)
                    .map_err(|_| Message::FontLoadFailed)
            );
        } else {
            // Fallback: try to load from included bytes if file doesn't exist
            // This helps during development when fonts might not be downloaded yet
            #[cfg(debug_assertions)]
            eprintln!("Warning: NotoSans-Regular.ttf not found in assets/fonts/. Run scripts/download-fonts.sh to download it.");
        }
        
        // Try to load Noto Color Emoji for emoji icons (try multiple possible names)
        let emoji_font_paths = [
            "assets/fonts/NotoColorEmoji.ttf",
            "assets/fonts/NotoEmoji-Regular.ttf",
        ];
        
        let mut emoji_loaded = false;
        for path in &emoji_font_paths {
            if let Ok(bytes) = std::fs::read(path) {
                font_commands.push(
                    iced::font::load(bytes)
                        .map(|_| Message::FontLoaded)
                        .map_err(|_| Message::FontLoadFailed)
                );
                emoji_loaded = true;
                break;
            }
        }
        
        if !emoji_loaded {
            #[cfg(debug_assertions)]
            eprintln!("Warning: Emoji font not found in assets/fonts/. Run scripts/download-fonts.sh to download it.");
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
