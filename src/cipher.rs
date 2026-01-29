use iced::{widget::{button, column, container, row, text, TextInput }, Element};

use crate::cipher::{caesar_cipher::Caesar, cipher_traits::CipherTraits, vigenere_cipher::Vigenere};

pub mod caesar_cipher;
pub mod vigenere_cipher;
pub mod cipher_traits;

#[derive(Debug, Clone)]
pub enum Message{
    Increment,
    Decrement,
    ContentChanged(String),
    InputChanged(String),
}


#[derive(Default)]
pub struct Cipher{
    caesar_cipher: Caesar,
    vigenere_cipher: Vigenere,

    to_encrypt: String,
    
    caesar_encrypted: String,
    vigenere_encrypted: String,
}


impl Cipher{
    pub fn view(cipher: &Cipher) -> Element<'_, Message> {
        container(
        column![
        row![
            button("Increment").on_press(Message::Increment),
            text(cipher.caesar_cipher.shift),
            button("Decrement").on_press(Message::Decrement),
        ],
        row![TextInput::new("keyword", &cipher.vigenere_cipher.keyword).on_input(Message::ContentChanged)],
        row![TextInput::new("text to encrypt", &cipher.to_encrypt).on_input(Message::InputChanged)],
        row![text("the caesar encrypted string "), text(&cipher.caesar_encrypted)],
        row![text("the vigenere encrypted string "), text(&cipher.vigenere_encrypted)]
        ]
        ).into()
    }

    pub fn update(cipher: &mut Cipher, message: Message) {
        match message{
            Message::Increment => {cipher.caesar_cipher.shift += 1;
                cipher.caesar_encrypted = Caesar::encrypt(&cipher.caesar_cipher, &cipher.to_encrypt)
            },
            Message::Decrement => {cipher.caesar_cipher.shift -= 1;
                cipher.caesar_encrypted = Caesar::encrypt(&cipher.caesar_cipher, &cipher.to_encrypt)
            },
            Message::ContentChanged(keyword) => {cipher.vigenere_cipher = Vigenere::new(keyword);
                cipher.vigenere_encrypted = Vigenere::encrypt(&cipher.vigenere_cipher, &cipher.to_encrypt)
            },
            Message::InputChanged(input) => cipher.to_encrypt = input
        }
    }
}
