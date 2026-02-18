use iced::widget::{button, text, text_input};
use iced::{
    Element,
    widget::{TextInput, column, container},
};

use crate::model::fence::{Percentage, PercentageError, parse_human_percentage};
use crate::ui::components::modal::modal;
use crate::ui::components::{card::card, fence_card::fence_card, layout::vert_stack};

use crate::{app::AppScreen, model::fence::Fence};

enum EditState {
    Idle,
    Editing { draft: EditDraft, index: usize },
    Creating { draft: EditDraft },
}

#[derive(Default)]
struct EditDraft {
    name: String,
    rep: String,
    lowest_markup: String,
    avg_markup: String,
    highest_markup: String,
    errors: EditErrors,
}

#[derive(Default)]
struct EditErrors {
    lowest: Option<PercentageError>,
    average: Option<PercentageError>,
    highest: Option<PercentageError>,
}

#[derive(Debug, Clone)]
pub enum FenceMessage {
    BaseInputChanged(String),
    ShowModal,
    HideModal,
    Name(String),
    Reputation(String),
    Lowest(String),
    Avg(String),
    Highest(String),
    Edit(usize),
    Submit,
}

pub struct FenceState {
    fences: Vec<Fence>,

    base_price_input: String,
    parsed_base_price: Option<u64>,

    error: Option<String>,

    create_fence_state: EditState,
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
            base_price_input: "".into(),
            parsed_base_price: Some(0),
            error: None,
            create_fence_state: EditState::Idle,
        }
    }
}

impl FenceState {
    fn current_draft_mut(&mut self) -> Option<&mut EditDraft> {
        match &mut self.create_fence_state {
            EditState::Creating { draft } => Some(draft),
            EditState::Editing { draft, .. } => Some(draft),
            EditState::Idle => None,
        }
    }
}

impl AppScreen for FenceState {
    type Msg = FenceMessage;

    fn view(&self) -> Element<'_, FenceMessage> {
        let col = vert_stack().push(column(self.fences.iter().enumerate().map(|(i, fence)| {
            fence_card(
                fence,
                self.parsed_base_price,
                &self.error,
                FenceMessage::Edit(i),
            )
        })));
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

        match &self.create_fence_state {
            EditState::Idle => base,
            EditState::Creating { draft } => {
                modal(base, fence_modal(draft, false), FenceMessage::HideModal)
            }
            EditState::Editing { draft, .. } => {
                modal(base, fence_modal(draft, true), FenceMessage::HideModal)
            }
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
            FenceMessage::Name(name) => {
                if let Some(draft) = self.current_draft_mut() {
                    draft.name = name;
                }
            }
            FenceMessage::Reputation(rep) => {
                if let Some(draft) = self.current_draft_mut() {
                    draft.rep = rep;
                }
            }
            FenceMessage::Lowest(low) => {
                if let Some(draft) = self.current_draft_mut() {
                    draft.lowest_markup = low;
                }
            }
            FenceMessage::Avg(avg) => {
                if let Some(draft) = self.current_draft_mut() {
                    draft.avg_markup = avg;
                }
            }
            FenceMessage::Highest(high) => {
                if let Some(draft) = self.current_draft_mut() {
                    draft.highest_markup = high;
                }
            }
            FenceMessage::ShowModal => {
                self.create_fence_state = EditState::Creating {
                    draft: EditDraft::default(),
                };
            }
            FenceMessage::Edit(index) => {
                if let Some(fence) = self.fences.get(index) {
                    self.create_fence_state = EditState::Editing {
                        index,
                        draft: EditDraft {
                            name: fence.name.clone(),
                            rep: fence.reputation.to_string(),
                            lowest_markup: fence.lowest_markup.to_string(),
                            avg_markup: fence.avg_markup.to_string(),
                            highest_markup: fence.highest_markup.to_string(),
                            errors: EditErrors::default(),
                        },
                    }
                }
            }
            FenceMessage::HideModal => self.create_fence_state = EditState::Idle,
            FenceMessage::Submit => {
                if let Some(draft) = self.current_draft_mut() {
                    let lowest = parse_human_percentage(&draft.lowest_markup);
                    let average = parse_human_percentage(&draft.avg_markup);
                    let highest = parse_human_percentage(&draft.highest_markup);

                    draft.errors = EditErrors {
                        lowest: lowest.as_ref().err().cloned(),
                        average: average.as_ref().err().cloned(),
                        highest: highest.as_ref().err().cloned(),
                    };

                    if let (Ok(rep), Ok(lowest), Ok(average), Ok(highest)) =
                        (draft.rep.parse::<u8>(), lowest, average, highest)
                    {
                        let new_fence = Fence {
                            name: draft.name.clone(),
                            reputation: rep,
                            lowest_markup: lowest,
                            avg_markup: average,
                            highest_markup: highest,
                        };

                        match &self.create_fence_state {
                            EditState::Creating { .. } => self.fences.push(new_fence),
                            EditState::Editing { index, .. } => {
                                if let Some(slot) = self.fences.get_mut(*index) {
                                    *slot = new_fence;
                                }
                            }
                            EditState::Idle => {}
                        }

                        self.create_fence_state = EditState::Idle;
                    }
                }
            }
        }
    }
}

fn fence_modal<'a>(draft: &EditDraft, is_editing: bool) -> Element<'a, FenceMessage> {
    let label = if is_editing { "Save" } else { "Create" };
    container(
        column![
            text("New Fence").size(24),
            column![
                column![
                    text("Name").size(12),
                    text_input("Merchant", &draft.name).on_input(FenceMessage::Name)
                ],
                column![
                    text("reputation").size(12),
                    text_input("enter a starting reputation", &draft.rep)
                        .on_input(FenceMessage::Reputation),
                ],
                column![
                    text("lowest").size(12),
                    text_input("enter a lowest markup price", &draft.lowest_markup)
                        .on_input(FenceMessage::Lowest),
                    match &draft.errors.lowest {
                        Some(err) => text(err.to_string()),
                        None => text(""),
                    }
                ],
                column![
                    text("avg").size(12),
                    text_input("enter an average markup price", &draft.avg_markup)
                        .on_input(FenceMessage::Avg),
                    match &draft.errors.average {
                        Some(err) => text(err.to_string()),
                        None => text(""),
                    }
                ],
                column![
                    text("highest").size(12),
                    text_input("enter a highest markup price", &draft.highest_markup)
                        .on_input(FenceMessage::Highest),
                    match &draft.errors.highest {
                        Some(err) => text(err.to_string()),
                        None => text(""),
                    }
                ]
                .spacing(5),
                button(label).on_press(FenceMessage::Submit),
            ]
            .spacing(10),
        ]
        .spacing(20),
    )
    .width(300)
    .padding(10)
    .style(container::rounded_box)
    .into()
}
