use iced::{Element, Length, widget::Container};

pub fn card<'a, Message: 'a>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    Container::new(content)
        .padding(16)
        .width(Length::Fill)
        .into()
}
