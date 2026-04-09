use iced::{
    widget::{container, text_editor},
    Element, Length, Font,
};

use crate::app::Message;
use crate::state::App;

pub fn editor<'a>(app: &'a App) -> Element<'a, Message> {
    // Get typography settings
    let typography = &app.editor_typography;
    
    // Create font based on selected font family
    // Note: Iced's font support is limited, so we use the first font in the fallback stack
    let font_family = typography.font_family.to_family_string();
    let font = Font::with_name(font_family);
    
    // Create a text editor with its own built-in scrolling
    // The text_editor widget handles scrolling internally, so we should NOT wrap it
    // in an outer scrollable container to avoid conflicts that cause crashes
    let editor = text_editor::TextEditor::new(&app.text_editor)
        .on_action(Message::EditorContentChanged)
        .font(font)
        .size(typography.font_size as f32)
        .line_height(typography.line_height)
        .height(Length::Fill);
    
    // Place the editor in a container with padding
    // The text_editor widget will handle its own scrolling
    container(editor)
        .padding([12, 20, 20, 20]) // Comfortable padding for coding
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
