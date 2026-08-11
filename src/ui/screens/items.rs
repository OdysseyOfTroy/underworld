use iced::Element;

use crate::{
    app::AppScreen,
    ui::components::{card::card, layout::vstack},
};

#[derive(Debug, Clone)]
pub enum ItemsMessage {}

#[derive(Default)]
pub struct ItemsState {}

impl AppScreen for ItemsState {
    type Msg = ItemsMessage;

    fn view(&self) -> Element<'_, ItemsMessage> {
        card(vstack())
    }

    fn update(&mut self, message: ItemsMessage) {}
}
