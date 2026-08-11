use crate::model::fence::{Fence, Percentage};
use crate::ui::components::layout::vstack;
use crate::ui::screens::fence::FenceMessage;
use iced::widget::{Column, Container, container};
use iced::{Background, Border, Color, Length, Theme};
use iced::{
    Element,
    widget::{Row, Text, button},
};

pub fn markup_string<'a>(
    label: & str,
    markup: Percentage,
    value: u64,
) -> Text<'a> {
Text::new(format!("{}: {} - {}", label, markup, value))
}

pub fn fence_card<'a>(
    fence: &Fence,
    base_price: Option<u64>,
    error: &'a Option<String>,
    on_edit: FenceMessage,
) -> Element<'a, FenceMessage> {
    let computed_prices = base_price.map(|base| {
        (
            fence.lowest_markup_price(base),
            fence.avg_markup_price(base),
            fence.highest_markup_price(base),
        )
    });
    Container::new(
        vstack()
            .push(Row::new().push(button("Edit").on_press(on_edit)))
            .push(if let Some((low, avg, high)) = computed_prices {
                Column::new()
                    .spacing(16)
                    .push(markup_string("Low Markup", fence.lowest_markup, low))
                    .push(markup_string("Avg Markup", fence.avg_markup, avg))
                    .push(markup_string("High Markup", fence.highest_markup, high))
            } else if let Some(error) = error {
                    Column::new().push(Text::new(error))
                } else {
                    Column::new().push(Text::new("Enter a valid base price")) 
                })
    )
    .style(|_theme: &Theme| container::Style {
        background: Some(Background::Color(Color::from_rgb8(48, 35, 28))),
        border: Border {
            width: 1.5,
            radius: 14.0.into(),
            color: Color::from_rgb8(135, 102, 62),
        },
        shadow: Default::default(),
        ..container::Style::default()
    })
    .padding(16)
    .width(Length::Fill)
    .into()
}
