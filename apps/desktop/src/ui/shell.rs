use iced::{Element, Length};
use crate::message::Message;
use crate::state::App;
use super::{
    activity_bar::activity_bar,
    assistant_panel::assistant_panel,
    editor_panel::editor_panel,
    explorer_panel::explorer_panel,
    status_bar::status_bar,
    top_bar::top_bar,
};

/// Main shell that composes all UI components
pub fn shell(app: &App) -> Element<'_, Message> {
    // Determine if AI panel should be visible
    let ai_panel_visible = matches!(app.active_activity, crate::state::Activity::Ai) || app.ai_panel_visible;
    
    // Build panels
    let top_bar = top_bar(app);
    let activity_bar = activity_bar(app);
    let explorer_panel = explorer_panel(app);
    let editor_panel = editor_panel(app);
    
    // Conditionally include AI panel
    let main_content = if ai_panel_visible {
        iced::widget::row![
            activity_bar.width(Length::Fixed(48.0)),
            explorer_panel.width(Length::Fixed(280.0)),
            editor_panel.width(Length::Fill),
            assistant_panel(app).width(Length::Fixed(320.0)),
        ]
        .height(Length::Fill)
    } else {
        iced::widget::row![
            activity_bar.width(Length::Fixed(48.0)),
            explorer_panel.width(Length::Fixed(280.0)),
            editor_panel.width(Length::Fill),
        ]
        .height(Length::Fill)
    };
    
    let status_bar = status_bar(app);
    
    // Combine everything
    iced::widget::column![
        top_bar.height(Length::Fixed(48.0)),
        main_content,
        status_bar.height(Length::Fixed(28.0)),
    ]
    .height(Length::Fill)
    .into()
}
