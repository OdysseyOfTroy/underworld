use iced::{
    Element,
    widget::{Column, column, text},
};

pub fn labeled<'a, Message: 'a>(
    label: &'a str,
    value: impl Into<Element<'a, Message>>,
) -> Column<'a, Message> {
    column![text(label), value.into()].spacing(6)
}
