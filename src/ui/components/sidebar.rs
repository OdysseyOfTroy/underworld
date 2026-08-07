use iced::{Background, Element, Length, Theme};
use iced::widget::{Button, Column, Container, button, container};

use crate::app::{Message, Screen};
use crate::ui::colours;

pub fn view(collapsed: bool, active: Screen) -> Element<'static, Message> {
    let toggle_button = Button::new(if collapsed { ">" } else { "<" })
        .on_press(Message::ToggleSidebar);

    let content = if collapsed {
        Column::new()
            .push(toggle_button)
    } else {
        Column::new()
            .spacing(10)
            .push(toggle_button)
            .push(nav_button("Fence", Screen::Fence, active))
            .push(nav_button("Cipher", Screen::Cipher, active))
    };

    Container::new(content)
        .padding(10)
        .height(Length::Fill)
        .width(if collapsed {
            Length::Shrink
        } else {
            Length::Fixed(180.0)
        })
        .style(|_theme: &Theme| container::Style {
            background: Some(Background::Color(colours::SIDEBAR_BG)),
            ..container::Style::default()
        })
        .into()
}

fn nav_button(label: &'static str, target: Screen, active: Screen) -> Button<'static, Message> {
    let is_active = target == active;

    Button::new(label)
        .width(Length::Fill)
        .on_press(Message::Navigate(target))
        .style(move |_theme: &Theme, status: button::Status| {
            let background = if is_active {
                colours::SIDEBAR_SELECTED_BG
            } else {
                match status {
                    button::Status::Hovered | button::Status::Pressed => colours::SIDEBAR_ITEM_HOVER,
                    _ => colours::SIDEBAR_ITEM_BG,
                }
            };

            button::Style {
                background: Some(Background::Color(background)),
                text_color: colours::SIDEBAR_TEXT,
                ..button::Style::default()
            }
        })
}
