use iced::{Element, Length, widget::{button, column, container, text}};
use crate::message::Message;
use crate::state::{App, Activity};
use super::style::StyleHelpers;

pub fn activity_bar(app: &App) -> Element<'_, Message> {
    let style = StyleHelpers::new(app.theme);
    
    let activities = [
        (Activity::Explorer, "📁", "Explorer"),
        (Activity::Search, "🔍", "Search"),
        (Activity::Ai, "🤖", "AI"),
        (Activity::SourceControl, "🔄", "Git"),
        (Activity::Settings, "⚙️", "Settings"),
    ];
    
    let activity_buttons: Vec<Element<_>> = activities
        .iter()
        .map(|(activity, icon, _tooltip)| {
            let is_active = app.active_activity == *activity;
            
            let message = if *activity == Activity::Ai {
                Message::ToggleAiPanel
            } else {
                Message::ActivitySelected(*activity)
            };
            
            let button = button(
                container(
                    text(*icon).size(18)
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
            )
            .width(Length::Fill)
            .height(Length::Fixed(48.0))
            .on_press(message)
            .style(iced::theme::Button::Custom(Box::new({
                let style = StyleHelpers::new(app.theme);
                let is_active = is_active;
                move || {
                    let mut appearance = if is_active {
                        style.primary_button()
                    } else {
                        style.secondary_button()
                    };
                    appearance.border.width = 0.0;
                    appearance.border.radius = 0.0.into();
                    if is_active {
                        appearance.background = Some(style.colors.selected_background.into());
                    }
                    appearance
                }
            })));
            
            button.into()
        })
        .collect();
    
    container(
        column(activity_buttons)
            .spacing(0)
            .width(Length::Fill)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(iced::theme::Container::Custom(Box::new(move || {
        let mut appearance = style.panel_container();
        appearance.border.width = 0.0;
        appearance.border.radius = 0.0.into();
        appearance
    })))
    .into()
}
