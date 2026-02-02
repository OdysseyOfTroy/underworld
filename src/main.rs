use crate::cipher::Cipher;

pub mod cipher;
pub mod fence;

fn main() -> iced::Result {
    iced::run(Cipher::update, Cipher::view)
}
