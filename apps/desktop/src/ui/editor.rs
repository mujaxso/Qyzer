use iced::{
    widget::{scrollable, container, text_editor},
    Element, Length, Font,
};

use crate::app::Message;

pub fn editor<'a>(editor_content: &'a iced::widget::text_editor::Content) -> Element<'a, Message> {
    // Create a simple text editor without line numbers for now
    let editor = text_editor::TextEditor::new(editor_content)
        .on_action(Message::EditorContentChanged)
        .font(Font::MONOSPACE)
        .height(Length::Fill);
    
    // Create a scrollable container for the editor
    // Use a fixed height container to prevent infinite layout issues
    let editor_container = container(editor)
        .padding(16)
        .width(Length::Fill)
        .height(Length::Fill);
    
    // Use a scrollable with explicit height
    scrollable(editor_container)
        .height(Length::Fill)
        .into()
}
