use iced::widget::{button, text, text_input};
use iced::{
    Element,
    widget::{TextInput, column, container},
};

use crate::model::fence::Percentage;
use crate::ui::components::modal::modal;
use crate::ui::components::{card::card, fence_card::fence_card, layout::vert_stack};

use crate::{app::AppScreen, model::fence::Fence};

#[derive(Clone, Debug)]
pub enum FenceMessage {
    BaseInputChanged(String),
    ShowModal,
    HideModal,
    Name(String),
    Reputation(String),
    Lowest(String),
    Avg(String),
    Highest(String),
}

pub struct FenceState {
    fences: Vec<Fence>,
    current_fence: Fence,

    show_create_fence_modal: bool,

    base_price_input: String,
    parsed_base_price: Option<u64>,

    error: Option<String>,

    raw_reputation: String,
    raw_lowest_markup: String,
    raw_avg_markup: String,
    raw_highest_markup: String,
}

impl Default for FenceState {
    fn default() -> Self {
        FenceState {
            fences: [
                Fence::default(),
                Fence::new(
                    "Dave",
                    10,
                    Percentage(1050),
                    Percentage(1125),
                    Percentage(1250),
                ),
            ]
            .to_vec(),
            current_fence: Fence::default(),
            show_create_fence_modal: false,
            base_price_input: "".into(),
            parsed_base_price: Some(0),
            error: None,
            raw_reputation: "".into(),
            raw_lowest_markup: "".into(),
            raw_avg_markup: "".into(),
            raw_highest_markup: "".into(),
        }
    }
}

impl AppScreen for FenceState {
    type Msg = FenceMessage;

    fn view(&self) -> Element<'_, FenceMessage> {
        let mut col = vert_stack();
        for fence in &self.fences {
            col = col.push(fence_card::<FenceMessage>(
                fence,
                self.parsed_base_price,
                &self.error,
            ))
        }
        let base = card(
            vert_stack()
                .push(button("Add").on_press(FenceMessage::ShowModal))
                .push(
                    TextInput::new("Enter base price", &self.base_price_input)
                        .padding(10)
                        .on_input(FenceMessage::BaseInputChanged),
                )
                .height(1200)
                .push(col),
        );
        if self.show_create_fence_modal {
            let content = container(
                column![
                    text("New Fence").size(24),
                    column![
                        column![
                            text("Name").size(12),
                            text_input("Merchant", &self.current_fence.name)
                                .on_input(FenceMessage::Name)
                        ],
                        column![
                            text("reputation").size(12),
                            text_input("enter a starting reputation", &self.raw_reputation)
                                .on_input(FenceMessage::Reputation),
                        ],
                        column![
                            text("lowest").size(12),
                            text_input("enter a lowest markup price", &self.raw_lowest_markup)
                                .on_input(FenceMessage::Lowest),
                        ],
                        column![
                            text("avg").size(12),
                            text_input("enter an average markup price", &self.raw_avg_markup)
                                .on_input(FenceMessage::Avg),
                        ],
                        column![
                            text("highest").size(12),
                            text_input("enter a highest markup price", &self.raw_highest_markup)
                                .on_input(FenceMessage::Highest),
                        ]
                        .spacing(5),
                        button(text("Create")).on_press(FenceMessage::HideModal),
                    ]
                    .spacing(10),
                ]
                .spacing(20),
            )
            .width(300)
            .padding(10)
            .style(container::rounded_box);
            modal(base, content, FenceMessage::HideModal)
        } else {
            base
        }
    }

    fn update(&mut self, message: FenceMessage) {
        match message {
            FenceMessage::BaseInputChanged(input) => {
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
            FenceMessage::Name(name) => self.current_fence.name = name.to_string(),
            FenceMessage::Reputation(rep) => {
                self.raw_reputation = rep;
                self.current_fence.reputation = self.raw_reputation.parse::<u8>().unwrap_or(0)
            }
            FenceMessage::Lowest(low) => {
                self.raw_lowest_markup = low;
                self.current_fence.lowest_markup =
                    Percentage(self.raw_lowest_markup.parse::<u64>().unwrap_or(1000))
            }
            FenceMessage::Avg(avg) => {
                self.raw_avg_markup = avg;
                self.current_fence.avg_markup =
                    Percentage(self.raw_avg_markup.parse::<u64>().unwrap_or(1000))
            }
            FenceMessage::Highest(high) => {
                self.raw_highest_markup = high;
                self.current_fence.highest_markup =
                    Percentage(self.raw_highest_markup.parse::<u64>().unwrap_or(1000))
            }
            FenceMessage::ShowModal => {
                self.show_create_fence_modal = true;
            }
            FenceMessage::HideModal => self.show_create_fence_modal = false,
        }
    }
}
