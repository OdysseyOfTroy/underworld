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
    vigenere_cipher: Option<Vigenere>,

    vigenere_keyword: String,

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
        row![TextInput::new("keyword", &cipher.vigenere_keyword).on_input(Message::ContentChanged)],
        row![TextInput::new("text to encrypt", &cipher.to_encrypt).on_input(Message::InputChanged)],
        row![text("the caesar encrypted string: "), text(&cipher.caesar_encrypted)],
        row![text("the vigenere encrypted string: "), text(&cipher.vigenere_encrypted)]
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
            Message::ContentChanged(keyword) => { match Vigenere::new(&keyword) {
            Ok(v) =>  cipher.vigenere_cipher = Some(v),
            Err(_) => cipher.vigenere_cipher = None,
            };
            cipher.vigenere_keyword = keyword;
            cipher.vigenere_encrypted = match &cipher.vigenere_cipher {
                Some(vigenere) => vigenere.encrypt(&cipher.to_encrypt),
                None => cipher.to_encrypt.clone(),
            };
            },
            Message::InputChanged(input) => {cipher.to_encrypt = input;
                cipher.caesar_encrypted = cipher.caesar_cipher.encrypt(&cipher.to_encrypt);
                cipher.vigenere_encrypted = match &cipher.vigenere_cipher {
                    Some(vigenere) => vigenere.encrypt(&cipher.to_encrypt),
                    None => cipher.to_encrypt.clone(),
                }
            }
        }
    }
}
