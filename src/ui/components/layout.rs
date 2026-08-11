use iced::{Background, Border, Element, Length, Theme, widget::{Column, Container, Row, container}};

use crate::ui::colours;

pub fn vstack<'a, Message>() -> Column<'a, Message> {
    Column::new().spacing(12)
}

pub fn hstack<'a, Message>() -> Row<'a, Message> {
    Row::new().spacing(12)
}

pub fn screen<'a, Message: 'a>(content: impl Into<Element<'a, Message>>) -> Container<'a, Message> {
    Container::new(content)
        .padding(16)
        .width(Length::Fill)
}

pub fn panel<'a, Message: 'a>(content: impl Into<Element<'a, Message>>) -> Container<'a, Message> {
    Container::new(content)
        .padding(16)
        .width(Length::Fill)
        .style(|_theme: &Theme| container::Style {
            background: Some(Background::Color(colours::SIDEBAR_BG)),
            border: Border {
                color: colours::SIDEBAR_ITEM_HOVER,
                width: 1.0,
                radius: 8.0.into(),
            },
            ..container::Style::default()
        })
}
