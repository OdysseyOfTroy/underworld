use crate::cipher::Cipher;

pub mod cipher;

fn main() -> iced::Result {
    iced::run(Cipher::update, Cipher::view)
}
