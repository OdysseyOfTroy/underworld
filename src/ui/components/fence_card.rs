use crate::model::fence::Fence;
use crate::ui::screens::fence::FenceMessage;
use crate::ui::components::{card::card, layout::vert_stack};
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
    card(
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
}
