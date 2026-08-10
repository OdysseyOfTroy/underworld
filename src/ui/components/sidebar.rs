use iced::widget::{Button, Column, Container, button, container, row, text};
use iced::{Background, Element, Length, Padding, Theme, alignment};
use iced_fonts::lucide;

use crate::app::{Message, Screen};
use crate::ui::colours;

pub const COLLAPSED_WIDTH: f32 = 48.0;
pub const EXPANDED_WIDTH: f32 = 180.0;

const SLOT: f32 = COLLAPSED_WIDTH - 20.0;

pub fn view(width: f32, collapsed: bool, active: Screen) -> Element<'static, Message> {
    let chevron = if collapsed {
        lucide::chevron_left()
    } else {
        lucide::chevron_right()
    };

    let toggle = Button::new(Container::new(chevron).center_x(Length::Fixed(SLOT)))
        .padding(Padding::from([8.0, 0.0]))
        .on_press(Message::ToggleSidebar);

    let inner = Column::new()
        .spacing(10)
        .width(Length::Fixed(EXPANDED_WIDTH - 20.0))
        .push(toggle)
        .push(nav_button(lucide::house(), "Home", Screen::Home, active))
        .push(nav_button(lucide::fence(), "Fence", Screen::Fence, active))
        .push(nav_button(
            lucide::feather(),
            "Cipher",
            Screen::Cipher,
            active,
        ))
        .push(nav_button(lucide::book(), "Items", Screen::Items, active));

    Container::new(inner)
        .padding(10)
        .height(Length::Fill)
        .width(Length::Fixed(width))
        .clip(true)
        .style(|_theme: &Theme| container::Style {
            background: Some(Background::Color(colours::SIDEBAR_BG)),
            ..container::Style::default()
        })
        .into()
}

fn nav_button(
    icon: impl Into<Element<'static, Message>>,
    label: &'static str,
    target: Screen,
    active: Screen,
) -> Button<'static, Message> {
    let is_active = target == active;

    let content = row![
        Container::new(icon).center_x(Length::Fixed(SLOT)),
        text(label),
    ]
    .spacing(8)
    .align_y(alignment::Vertical::Center);

    Button::new(content)
        .width(Length::Fixed(EXPANDED_WIDTH - 20.0))
        .padding(Padding::from([8.0, 0.0]))
        .on_press(Message::Navigate(target))
        .style(move |_theme: &Theme, status: button::Status| {
            let background = if is_active {
                colours::SIDEBAR_SELECTED_BG
            } else {
                match status {
                    button::Status::Hovered | button::Status::Pressed => {
                        colours::SIDEBAR_ITEM_HOVER
                    }
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
