use iced::widget::{button, text, Column, column};

use crate::cipher::{caesar_cipher::Caesar, vigenere_cipher::Vigenere};

pub mod caesar_cipher;
pub mod vigenere_cipher;
pub mod cipher_traits;

#[derive(Debug, Clone)]
pub enum Message{
    Increment,
    Decrement,
}


#[derive(Default)]
pub struct Cipher{
    caesar_cipher: Caesar,
}


impl Cipher{
    pub fn view(cipher: &Cipher) -> Column<'_, Message> {
        column![
            button("Increment").on_press(Message::Increment),
            text(cipher.caesar_cipher.shift),
            button("Decrement").on_press(Message::Decrement),
        ]
    }

    pub fn update(cipher: &mut Cipher, message: Message) {
        match message{
            Message::Increment => cipher.caesar_cipher.shift += 1,
            Message::Decrement => cipher.caesar_cipher.shift -= 1,
        }
    }
}
