use iced::{
    widget::{button, column, container, horizontal_space, row, text, text_input},
    Alignment, Element, Length,
};

use crate::message::Message;

pub fn top_bar<'a>(workspace_path: &'a str, is_dirty: bool) -> Element<'a, Message> {
    let status_indicator: Element<_> = if is_dirty {
        row![
            text("●").size(12).style(iced::theme::Text::Color(iced::Color::from_rgb8(255, 180, 0))),
            text("Unsaved").size(12).style(iced::theme::Text::Color(iced::Color::from_rgb8(220, 220, 220)))
        ]
        .spacing(4)
        .align_items(Alignment::Center)
        .into()
    } else {
        row![
            text("✓").size(12).style(iced::theme::Text::Color(iced::Color::from_rgb8(0, 200, 100))),
            text("Saved").size(12).style(iced::theme::Text::Color(iced::Color::from_rgb8(180, 180, 180)))
        ]
        .spacing(4)
        .align_items(Alignment::Center)
        .into()
    };

    row![
        // Logo/brand
        row![
            text("Q").size(20).style(iced::theme::Text::Color(iced::Color::from_rgb8(100, 160, 255))),
            text("yzer").size(20).style(iced::theme::Text::Color(iced::Color::from_rgb8(220, 220, 230))),
        ]
        .spacing(0)
        .align_items(Alignment::Center),
        horizontal_space(),
        // Workspace path display with manual entry option
        if workspace_path.is_empty() {
            // When no workspace is open, show an input field for manual entry
            container(
                column![
                    row![
                        text_input("Enter workspace path manually...", workspace_path)
                            .on_input(Message::WorkspacePathChanged)
                            .on_submit(Message::SubmitManualWorkspacePath(workspace_path.to_string()))
                            .padding([10, 12])
                            .width(Length::Fill)
                            .style(iced::theme::TextInput::Default),
                        button("Open")
                            .on_press(Message::SubmitManualWorkspacePath(workspace_path.to_string()))
                            .padding([10, 14])
                            .style(iced::theme::Button::Secondary),
                    ]
                    .spacing(8)
                    .align_items(Alignment::Center),
                    text("Or use the file picker (uses system theme)")
                        .size(10)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150)))
                ]
                .spacing(4)
            )
            .width(Length::FillPortion(3))
            .into()
        } else {
            // When workspace is open, show it as read-only
            container(
                container(
                    text(workspace_path)
                        .size(14)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(220, 220, 220)))
                )
                .padding([10, 12])
                .width(Length::Fill)
                .style(iced::theme::Container::Box)
            )
            .width(Length::FillPortion(3))
            .style(iced::theme::Container::Box)
        },
        // Buttons
        row![
            button("Open Workspace...")
                .on_press(Message::OpenWorkspace)
                .padding([8, 14])
                .style(iced::theme::Button::Secondary),
            button("Refresh")
                .on_press(Message::RefreshWorkspace)
                .padding([8, 14])
                .style(iced::theme::Button::Secondary),
        ]
        .spacing(8),
        horizontal_space(),
        // Status indicator
        container(status_indicator)
            .padding([6, 12])
            .style(iced::theme::Container::Box),
        // Save button
        button("Save")
            .on_press(Message::SaveFile)
            .padding([10, 18])
            .style(iced::theme::Button::Primary),
    ]
    .padding([12, 20])
    .align_items(Alignment::Center)
    .into()
}
