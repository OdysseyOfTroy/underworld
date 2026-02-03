use iced::{widget::{container, column, row, button, text, TextInput}, Element};

use crate::model::cipher::{caesar_cipher::Caesar, vigenere_cipher::Vigenere, cipher_traits::CipherTraits};

#[derive(Debug, Clone)]
pub enum Message {
    Increment,
    Decrement,
    ContentChanged(String),
    InputChanged(String),
}

#[derive(Default)]
pub struct CipherState {
    caesar_cipher: Caesar,
    vigenere_cipher: Option<Vigenere>,

    vigenere_keyword: String,

    to_encrypt: String,

    caesar_encrypted: String,
    vigenere_encrypted: String,
}

impl CipherState {
    pub fn view(&self) -> Element<'_, Message> {
        container(column![
            row![
                button("Increment").on_press(Message::Increment),
                text(self.caesar_cipher.shift),
                button("Decrement").on_press(Message::Decrement),
            ],
            row![
                TextInput::new("keyword", &self.vigenere_keyword)
                    .on_input(Message::ContentChanged)
            ],
            row![
                TextInput::new("text to encrypt", &self.to_encrypt)
                    .on_input(Message::InputChanged)
            ],
            row![
                text("the caesar encrypted string: "),
                text(&self.caesar_encrypted)
            ],
            row![
                text("the vigenere encrypted string: "),
                text(&self.vigenere_encrypted)
            ]
        ])
        .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.caesar_cipher.shift += 1;
                self.caesar_encrypted = Caesar::encrypt(&self.caesar_cipher, &self.to_encrypt)
            }
            Message::Decrement => {
                self.caesar_cipher.shift -= 1;
                self.caesar_encrypted = Caesar::encrypt(&self.caesar_cipher, &self.to_encrypt)
            }
            Message::ContentChanged(keyword) => {
                match Vigenere::new(&keyword) {
                    Ok(v) => self.vigenere_cipher = Some(v),
                    Err(_) => self.vigenere_cipher = None,
                };
                self.vigenere_keyword = keyword;
                self.vigenere_encrypted = match &self.vigenere_cipher {
                    Some(vigenere) => vigenere.encrypt(&self.to_encrypt),
                    None => self.to_encrypt.clone(),
                };
            }
            Message::InputChanged(input) => {
                self.to_encrypt = input;
                self.caesar_encrypted = self.caesar_cipher.encrypt(&self.to_encrypt);
                self.vigenere_encrypted = match &self.vigenere_cipher {
                    Some(vigenere) => vigenere.encrypt(&self.to_encrypt),
                    None => self.to_encrypt.clone(),
                }
            }
        }
    }
}
