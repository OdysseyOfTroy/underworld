use crate::model::fence::Fence;
use crate::ui::components::layout::vert_stack;
use crate::ui::screens::fence::FenceMessage;
use iced::widget::{Container, container};
use iced::{Background, Border, Color, Length, Theme};
use iced::{
    Element,
    widget::{Row, Text, button},
};

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
        vert_stack()
            .push(Row::new().push(button("Edit").on_press(on_edit)))
            .push(
                Row::new()
                    .spacing(20)
                    .push(Text::new(format!("Low markup: {}", fence.lowest_markup)))
                    .push(Text::new(format!("Avg markup: {}", fence.avg_markup)))
                    .push(Text::new(format!("High markup: {}", fence.highest_markup))),
            )
            .push(if let Some((low, avg, high)) = computed_prices {
                Row::new()
                    .spacing(20)
                    .push(Text::new(format!("Low: {}", low)))
                    .push(Text::new(format!("Avg: {}", avg)))
                    .push(Text::new(format!("High: {}", high)))
            } else if let Some(error) = error {
                Row::new().push(Text::new(error))
            } else {
                Row::new().push(Text::new("Enter a valid base price"))
            }),
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
