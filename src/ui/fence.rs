use iced::{
    widget::{Column, Row, Text, TextInput},
    Element,
};

use crate::model::fence::Fence;

#[derive(Clone, Debug)]
pub enum Message {
   BaseInputChanged(String), 
}

#[derive(Default)]
pub struct FenceState {
    fence: Fence,

    base_price_input: String,
    parsed_base_price: Option<u64>,

    error: Option<String>,
}

impl FenceState {
    pub fn view(&self) -> Element<'_, Message> {
        
    let computed_prices = self.parsed_base_price.map(|base| {
            (
        self.fence.lowest_markup_price(base),
        self.fence.avg_markup_price(base),
        self.fence.highest_markup_price(base),
        )
     });
        Column::new()
            .push(
                TextInput::new(
                    "Enter base price",
                    &self.base_price_input,
                )
                .padding(10)
                .on_input(Message::BaseInputChanged),
            )
            .push(
                Row::new()
                    .spacing(20)
                    .push(Text::new(format!("Low markup: {}", self.fence.lowest_markup)))
                    .push(Text::new(format!("Avg markup: {}", self.fence.avg_markup)))
                    .push(Text::new(format!("High markup: {}", self.fence.highest_markup))),
            )
            .push(
                if let Some((low, avg, high)) = computed_prices {
                    Row::new()
                        .spacing(20)
                        .push(Text::new(format!("Low: {}", low)))
                        .push(Text::new(format!("Avg: {}", avg)))
                        .push(Text::new(format!("High: {}", high)))
                } else if let Some(error) = &self.error {
                    Row::new().push(Text::new(error))
                } else {
                    Row::new().push(Text::new("Enter a valid base price"))
                },
            )
            .into()
        }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::BaseInputChanged(input) => {
               self.base_price_input = input.clone();
               match input.parse::<u64>() {
                    Ok(value) => {
                        self.parsed_base_price = Some(value);
                        self.error = None;
                    }
                    Err(_) => {
                        self.parsed_base_price = None;
                        self.error = Some("Invalid number".into());
                    }
                }
            }
        }
    }
}
