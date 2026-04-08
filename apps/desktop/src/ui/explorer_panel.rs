use iced::{Element, Length, widget::{button, column, container, row, scrollable, text}};
use crate::message::Message;
use crate::state::App;
use super::style::StyleHelpers;

pub fn explorer_panel(app: &App) -> Element<'_, Message> {
    let style = StyleHelpers::new(app.theme);
    
    let header = container(
        row![
            text("EXPLORER")
                .size(11)
                .style(style.text_muted()),
            iced::widget::horizontal_space(Length::Fill),
            button(
                text("⟳").size(14)
            )
            .on_press(Message::RefreshWorkspace)
            .padding([4, 8])
            .style(iced::theme::Button::Custom(Box::new(move || style.secondary_button())))
        ]
        .align_items(iced::Alignment::Center)
    )
    .padding([12, 16])
    .width(Length::Fill);
    
    let content = if app.file_entries.is_empty() {
        container(
            column![
                text("No files in workspace")
                    .style(style.text_muted()),
                button("Open Workspace")
                    .on_press(Message::OpenWorkspace)
                    .padding(8)
                    .style(iced::theme::Button::Custom(Box::new(move || style.secondary_button())))
            ]
            .spacing(12)
            .align_items(iced::Alignment::Center)
        )
        .center_y()
        .center_x()
        .width(Length::Fill)
        .height(Length::Fill)
    } else {
        let entries: Vec<Element<_>> = app.file_entries
            .iter()
            .enumerate()
            .map(|(idx, entry)| {
                let is_selected = app.active_file_path.as_ref() == Some(&entry.path);
                
                let icon = if entry.is_dir { "📁" } else { "📄" };
                let text_color = if is_selected {
                    style.text_on_accent()
                } else if entry.is_dir {
                    style.text_primary()
                } else {
                    style.text_secondary()
                };
                
                let row_content = row![
                    text(icon).size(14),
                    text(&entry.name)
                        .size(13)
                        .style(text_color),
                ]
                .spacing(8)
                .align_items(iced::Alignment::Center);
                
                let message = if entry.is_dir {
                    Message::ToggleDirectory(entry.path.clone())
                } else {
                    Message::FileSelected(idx)
                };
                
                container(
                    button(row_content)
                        .on_press(message)
                        .padding([6, 12])
                        .width(Length::Fill)
                        .style(iced::theme::Button::Custom(Box::new({
                            let style = StyleHelpers::new(app.theme);
                            let is_selected = is_selected;
                            move || {
                                let mut appearance = style.secondary_button();
                                appearance.border.width = 0.0;
                                if is_selected {
                                    appearance.background = Some(style.colors.selected_background.into());
                                }
                                appearance
                            }
                        })))
                )
                .into()
            })
            .collect();
        
        scrollable(
            column(entries)
                .spacing(0)
                .width(Length::Fill)
        )
        .height(Length::Fill)
    };
    
    container(
        column![
            header,
            content.height(Length::Fill),
        ]
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(iced::theme::Container::Custom(Box::new(move || style.panel_container())))
    .into()
}
