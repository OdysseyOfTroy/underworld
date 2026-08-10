use iced::Element;

use crate::{
    app::AppScreen,
    ui::components::{card::card, layout::vert_stack},
};

#[derive(Debug, Clone)]
pub enum ItemsMessage {}

#[derive(Default)]
pub struct ItemsState {}

impl AppScreen for ItemsState {
    type Msg = ItemsMessage;

    fn view(&self) -> Element<'_, ItemsMessage> {
        card(vert_stack()).into()
    }

    fn update(&mut self, message: ItemsMessage) {}
}
