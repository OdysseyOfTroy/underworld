use iced::{
    Element, Font, widget::{Column, Text, column, text},
};

pub fn labeled<'a, Message: 'a>(
    label: &'a str,
    value: impl Into<Element<'a, Message>>,
) -> Column<'a, Message> {
    column![text(label), value.into()].spacing(6)
}

pub fn h1<'a>(content: impl text::IntoFragment<'a>) -> Text<'a> {
    text(content).size(28.0).font(Font {
        weight: iced::font::Weight::Bold,
        ..Font::DEFAULT
    })
}

pub fn h2<'a>(content: impl text::IntoFragment<'a>) -> Text<'a> {
    text(content).size(20.0).font(Font {
        weight: iced::font::Weight::Semibold,
        ..Font::DEFAULT
    })
}
