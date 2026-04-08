use iced::{Element, Length, widget::{column, container, row, scrollable, text}};
use crate::message::Message;
use crate::state::App;
use super::style::StyleHelpers;
use super::editor;

pub fn editor_panel(app: &App) -> Element<'_, Message> {
    let style = StyleHelpers::new(app.theme);
    
    let header = if let Some(active_path) = &app.active_file_path {
        let file_name = active_path.split('/').last().unwrap_or(active_path);
        container(
            row![
                text("📄").size(14),
                text(file_name)
                    .size(13)
                    .style(style.text_primary()),
                iced::widget::horizontal_space(Length::Fill),
                if app.is_dirty {
                    text("● Unsaved").size(11).style(style.text_warning())
                } else {
                    text("✓ Saved").size(11).style(style.text_success())
                }
            ]
            .spacing(8)
            .align_items(iced::Alignment::Center)
        )
        .padding([12, 16])
        .width(Length::Fill)
    } else {
        container(
            text("No file selected")
                .style(style.text_muted())
        )
        .padding([12, 16])
        .width(Length::Fill)
    };
    
    let editor_content = if let Some(_) = &app.active_file_path {
        // Use the existing editor component
        editor::editor(&app.text_editor)
    } else {
        // Welcome screen
        container(
            column![
                text("Neote").size(32).style(style.text_primary()),
                text("AI‑first IDE").size(16).style(style.text_secondary()),
                container(iced::widget::horizontal_rule(1)).width(150),
                column![
                    text("Open a file from the explorer").style(style.text_muted()),
                    text("Ask AI about the workspace").style(style.text_muted()),
                ]
                .spacing(8)
                .padding(16),
            ]
            .spacing(16)
            .align_items(iced::Alignment::Center)
        )
        .center_y()
        .center_x()
        .width(Length::Fill)
        .height(Length::Fill)
    };
    
    container(
        column![
            header,
            editor_content.height(Length::Fill),
        ]
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(iced::theme::Container::Custom(Box::new(move || {
        let mut appearance = style.panel_container();
        appearance.border.width = 0.0;
        appearance
    })))
    .into()
}
