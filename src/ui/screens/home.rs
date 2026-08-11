use iced::Element;

use crate::{
    app::AppScreen,
    ui::components::{card::card, layout::vstack},
};

#[derive(Debug, Clone)]
pub enum HomeMessage {}

#[derive(Default)]
pub struct HomeState {}

impl AppScreen for HomeState {
    type Msg = HomeMessage;

    fn view(&self) -> Element<'_, HomeMessage> {
        card(vstack())
    }

    fn update(&mut self, message: HomeMessage) {}
}
