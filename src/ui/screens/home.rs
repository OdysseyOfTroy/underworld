use iced::Element;

use crate::{
    app::AppScreen,
    ui::components::{card::card, layout::vert_stack},
};

#[derive(Debug, Clone)]
pub enum HomeMessage {}

pub struct HomeState {}

impl Default for HomeState {
    fn default() -> Self {
        HomeState {}
    }
}

impl AppScreen for HomeState {
    type Msg = HomeMessage;

    fn view(&self) -> Element<'_, HomeMessage> {
        card(vert_stack()).into()
    }

    fn update(&mut self, message: HomeMessage) {}
}
