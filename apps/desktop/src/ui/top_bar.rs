use iced::{Element, Length, widget::{button, container, row, text, text_input}};
use crate::message::Message;
use crate::state::App;
use super::style::StyleHelpers;

pub fn top_bar(app: &App) -> Element<'_, Message> {
    let style = StyleHelpers::new(app.theme);
    
    // Workspace path input
    let workspace_input = text_input(
        "Workspace path...",
        &app.workspace_path,
    )
    .on_input(Message::WorkspacePathChanged)
    .padding([8, 12])
    .width(Length::FillPortion(3));
    
    // Open workspace button
    let open_button = button(
        text("Open")
    )
    .on_press(Message::OpenWorkspace)
    .padding([8, 16]);
    
    // Save button
    let save_button = button(
        text("Save")
    )
    .on_press(Message::SaveFile)
    .padding([8, 16]);
    
    // Status indicator
    let status_indicator = if app.is_dirty {
        container(
            row![
                text("●").size(12).style(style.text_warning()),
                text("Unsaved").size(12).style(style.text_secondary()),
            ]
            .spacing(4)
            .align_items(iced::Alignment::Center)
        )
        .padding([6, 12])
    } else {
        container(
            row![
                text("✓").size(12).style(style.text_success()),
                text("Saved").size(12).style(style.text_muted()),
            ]
            .spacing(4)
            .align_items(iced::Alignment::Center)
        )
        .padding([6, 12])
    };
    
    container(
        row![
            // Logo/brand
            container(
                row![
                    text("N").size(20).style(style.text_primary()),
                    text("eote").size(20).style(style.text_secondary()),
                ]
                .spacing(0)
            )
            .padding([0, 16]),
            
            iced::widget::horizontal_space(Length::Fill),
            
            // Workspace controls
            container(
                row![
                    workspace_input,
                    open_button,
                    button(
                        text("⟳")
                    )
                    .on_press(Message::RefreshWorkspace)
                    .padding([8, 12]),
                ]
                .spacing(8)
                .align_items(iced::Alignment::Center)
            )
            .width(Length::FillPortion(2)),
            
            iced::widget::horizontal_space(Length::Fill),
            
            // Status and save
            container(
                row![
                    status_indicator,
                    save_button,
                ]
                .spacing(8)
                .align_items(iced::Alignment::Center)
            ),
        ]
        .align_items(iced::Alignment::Center)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(iced::theme::Container::Custom(Box::new(move || {
        let mut appearance = style.elevated_container();
        appearance.border.width = 0.0;
        appearance.border.radius = 0.0.into();
        appearance
    })))
    .into()
}
