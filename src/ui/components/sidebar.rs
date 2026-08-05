use iced::{Element, Length};
use iced::widget::{Button, Column, Container};

use crate::app::{Message, Screen};

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
        .width(if collapsed {
            Length::Shrink
        } else {
            Length::Fixed(180.0)
        })
        .into()
}

fn nav_button(label: &'static str, target: Screen, active: Screen) -> Button<'static, Message> {
    let btn = Button::new(label).on_press(Message::Navigate(target));

    // Later: style differently when target == active
    if target == active {
        btn // apply active styling here
    } else {
        btn
    }
}
