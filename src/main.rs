use crate::app::App;

pub mod app;
pub mod model;
pub mod ui;
pub mod db;

#[tokio::main]
async fn main() -> iced::Result {
    let pool = db::init_pool("underworld.db").await
        .expect("Failed to initialise database");
    iced::application(move || App::new(pool.clone()), App::update, App::view)
        .title(App::title)
        .run()
}
