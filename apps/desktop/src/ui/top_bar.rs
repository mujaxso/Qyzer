use iced::{Element, Length, widget::{button, container, row, text, text_input}};
use crate::message::Message;
use crate::state::App;
use super::style::StyleHelpers;

pub fn top_bar(app: &App) -> Element<'_, Message> {
    let style = StyleHelpers::new(app.theme);
    
    // Compact workspace path input
    let workspace_input = text_input(
        "Workspace path...",
        &app.workspace_path,
    )
    .on_input(Message::WorkspacePathChanged)
    .padding([6, 10])
    .width(Length::FillPortion(3))
    .style(iced::theme::TextInput::Custom(Box::new(move |_theme| {
        iced::widget::text_input::Appearance {
            background: style.colors.input_background.into(),
            border: iced::Border {
                color: style.colors.border,
                width: 1.0,
                radius: crate::ui::common::RADIUS_SM.into(),
            },
            icon_color: style.colors.text_muted,
        }
    })));
    
    // Compact buttons
    let open_button = button(
        text("Open").size(13)
    )
    .on_press(Message::OpenWorkspace)
    .padding([6, 12])
    .style(iced::theme::Button::Secondary);
    
    let save_button = button(
        text("Save").size(13)
    )
    .on_press(Message::SaveFile)
    .padding([6, 12])
    .style(iced::theme::Button::Primary);
    
    // Subtle status indicator
    let status_indicator = if app.is_dirty {
        container(
            row![
                text("●").size(10).style(iced::theme::Text::Color(style.colors.warning)),
                text("Unsaved").size(11).style(iced::theme::Text::Color(style.colors.text_secondary)),
            ]
            .spacing(4)
            .align_items(iced::Alignment::Center)
        )
        .padding([4, 8])
        .style(iced::theme::Container::Custom(Box::new(move |_theme| {
            container::Appearance {
                background: Some(style.colors.elevated_panel_background.into()),
                border: iced::Border {
                    color: style.colors.border,
                    width: 1.0,
                    radius: crate::ui::common::RADIUS_SM.into(),
                },
                ..Default::default()
            }
        })))
    } else {
        container(
            row![
                text("✓").size(10).style(iced::theme::Text::Color(style.colors.success)),
                text("Saved").size(11).style(iced::theme::Text::Color(style.colors.text_muted)),
            ]
            .spacing(4)
            .align_items(iced::Alignment::Center)
        )
        .padding([4, 8])
        .style(iced::theme::Container::Custom(Box::new(move |_theme| {
            container::Appearance {
                background: Some(style.colors.elevated_panel_background.into()),
                border: iced::Border {
                    color: style.colors.border,
                    width: 1.0,
                    radius: crate::ui::common::RADIUS_SM.into(),
                },
                ..Default::default()
            }
        })))
    };
    
    container(
        row![
            // Minimal logo/brand
            container(
                row![
                    text("N").size(18).style(iced::theme::Text::Color(style.colors.accent)),
                    text("eote").size(18).style(iced::theme::Text::Color(style.colors.text_primary)),
                ]
                .spacing(0)
            )
            .padding([0, 12]),
            
            iced::widget::horizontal_space(),
            
            // Workspace controls - compact
            container(
                row![
                    workspace_input,
                    open_button,
                    button(
                        text("⟳").size(14)
                    )
                    .on_press(Message::RefreshWorkspace)
                    .padding([6, 10])
                    .style(iced::theme::Button::Secondary),
                ]
                .spacing(6)
                .align_items(iced::Alignment::Center)
            )
            .width(Length::FillPortion(2)),
            
            iced::widget::horizontal_space(),
            
            // Status and save - compact
            container(
                row![
                    status_indicator,
                    save_button,
                ]
                .spacing(6)
                .align_items(iced::Alignment::Center)
            ),
        ]
        .align_items(iced::Alignment::Center)
        .padding([0, 8])
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(iced::theme::Container::Custom(Box::new(move |_theme| {
        container::Appearance {
            background: Some(style.colors.shell_background.into()),
            border: iced::Border {
                color: style.colors.border,
                width: 0.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        }
    })))
    .into()
}
