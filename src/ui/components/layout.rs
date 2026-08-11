use iced::{
    Background, Border, Element, Length, Size, Theme, widget::{Column, Container, Row, container, responsive},
};

use crate::ui::colours;

pub fn vstack<'a, Message>() -> Column<'a, Message> {
    Column::new().spacing(12)
}

pub fn hstack<'a, Message>() -> Row<'a, Message> {
    Row::new().spacing(12)
}

pub fn screen<'a, Message: 'a>(content: impl Into<Element<'a, Message>>) -> Container<'a, Message> {
    Container::new(content).padding(16).width(Length::Fill)
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

pub fn grid<'a, Message, F>(
    count: usize,
    min_width: f32,
    max_width: f32,
    spacing: f32,
    item: F,
) -> Element<'a, Message>
where 
    Message: 'a,
    F: Fn(usize) -> Element<'a, Message> + 'a,
{
    responsive(move |size: Size| {
let cols = (((size.width + spacing) / (max_width + spacing)).floor() as usize).max(1);

        let item_width = if cols == 1 {
            size.width.clamp(min_width, max_width)
        } else {
            max_width
        };

        let mut column = Column::new().spacing(spacing);
        let mut row = Row::new().spacing(spacing);

        for i in 0..count {
            row = row.push(Container::new(item(i)).width(Length::Fixed(item_width)));

            if (i + 1) % cols == 0 {
                column = column.push(row);
                row = Row::new().spacing(spacing);
            }
        }

        if !count.is_multiple_of(cols) {
            column = column.push(row);
        }

        column.into()
    }).into()
}
