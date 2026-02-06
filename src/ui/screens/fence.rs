use iced::widget::{button, text};
use iced::{Element, widget::TextInput};

use crate::model::fence::Percentage;
use crate::ui::components::modal::modal;
use crate::ui::components::{card::card, fence_card::fence_card, layout::vert_stack};

use crate::{app::AppScreen, model::fence::Fence};

#[derive(Clone, Debug)]
pub enum FenceMessage {
    BaseInputChanged(String),
    ShowModal,
    HideModal,
}

pub struct FenceState {
    fences: Vec<Fence>,

    show_create_fence_modal: bool,

    base_price_input: String,
    parsed_base_price: Option<u64>,

    error: Option<String>,
}

impl Default for FenceState {
    fn default() -> Self {
        FenceState {
            fences: [
                Fence::default(),
                Fence::new(10, Percentage(1050), Percentage(1125), Percentage(1250)),
            ]
            .to_vec(),
            show_create_fence_modal: false,
            base_price_input: "".into(),
            parsed_base_price: Some(0),
            error: None,
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
                .push(col),
        );
        if self.show_create_fence_modal {
            let content = vert_stack().push(text("something"));
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
            FenceMessage::ShowModal => {
                self.show_create_fence_modal = true;
            }
            FenceMessage::HideModal => {
                self.show_create_fence_modal = false
            }
        }
    }
}
