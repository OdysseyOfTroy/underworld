use crate::app::App;

pub mod app;
pub mod db;
pub mod model;
pub mod ui;

#[tokio::main]
async fn main() -> iced::Result {
    let pool = db::init_pool("underworld.db")
        .await
        .expect("Failed to initialise database");
    iced::application(move || App::new(pool.clone()), App::update, App::view)
        .title("Thieves Toolkit")
        .subscription(App::subscription)
        .font(iced_fonts::LUCIDE_FONT_BYTES)
        .run()
}
