use iced::{
    widget::{button, row, text},
    Alignment, Element, Length,
};
use iced::widget::text_editor;

use crate::app::Message;

pub fn header<'a>(active_file_path: Option<&'a String>, is_dirty: bool) -> Element<'a, Message> {
    row![
        text(
            active_file_path
                .map(|p| format!("Editing: {}", p))
                .unwrap_or_else(|| "No file selected".to_string())
        )
        .size(16),
        if is_dirty {
            text(" (modified)").size(16).style(iced::Color::from_rgb8(255, 165, 0))
        } else {
            text(" (saved)").size(16).style(iced::Color::from_rgb8(0, 128, 0))
        },
        button("Save").on_press(Message::SaveFile).padding(8),
    ]
    .spacing(10)
    .align_items(Alignment::Center)
    .into()
}

pub fn editor<'a>(editor_content: &'a str) -> Element<'a, Message> {
    // Use text_editor widget for proper multi-line editing
    text_editor(editor_content)
        .on_action(|action| match action {
            iced::widget::text_editor::Action::Edit(action) => {
                Message::EditorContentChanged(action.value)
            }
            _ => Message::EditorContentChanged(editor_content.to_string()),
        })
        .height(Length::Fill)
        .into()
}
