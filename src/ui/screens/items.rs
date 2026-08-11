use iced::Element;

use crate::{
    app::AppScreen,
    ui::components::layout::{screen, vstack},
};

#[derive(Debug, Clone)]
pub enum ItemsMessage {}

#[derive(Default)]
pub struct ItemsState {}

impl AppScreen for ItemsState {
    type Msg = ItemsMessage;

    fn view(&self) -> Element<'_, ItemsMessage> {
        screen(vstack()).into()
    }

    fn update(&mut self, message: ItemsMessage) {}
}
