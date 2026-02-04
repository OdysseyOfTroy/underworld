use crate::app::App;

pub mod app;
pub mod model;
pub mod ui;

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title(App::title)
        .run()
}
