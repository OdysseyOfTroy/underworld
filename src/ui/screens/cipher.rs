use iced::{
    Element,
    widget::{TextInput, button, row, text},
};

use crate::ui::components::{card::card, layout::vert_stack};

use crate::{
    app::AppScreen,
    model::cipher::{
        caesar_cipher::Caesar, cipher_traits::CipherTraits, vigenere_cipher::Vigenere,
    },
};

#[derive(Debug, Clone)]
pub enum CipherMessage {
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

impl AppScreen for CipherState {
    type Msg = CipherMessage;

    fn view(&self) -> Element<'_, CipherMessage> {
        card(
            vert_stack()
                .push(row![
                    button("Increment").on_press(CipherMessage::Increment),
                    text(self.caesar_cipher.shift),
                    button("Decrement").on_press(CipherMessage::Decrement),
                ])
                .push(row![
                    TextInput::new("keyword", &self.vigenere_keyword)
                        .on_input(CipherMessage::ContentChanged)
                ])
                .push(row![
                    TextInput::new("text to encrypt", &self.to_encrypt)
                        .on_input(CipherMessage::InputChanged)
                ])
                .push(row![
                    text("the caesar encrypted string: "),
                    text(&self.caesar_encrypted)
                ])
                .push(row![
                    text("the vigenere encrypted string: "),
                    text(&self.vigenere_encrypted)
                ]),
        )
    }

    fn update(&mut self, message: CipherMessage) {
        match message {
            CipherMessage::Increment => {
                self.caesar_cipher.shift += 1;
                self.caesar_encrypted = Caesar::encrypt(&self.caesar_cipher, &self.to_encrypt)
            }
            CipherMessage::Decrement => {
                self.caesar_cipher.shift -= 1;
                self.caesar_encrypted = Caesar::encrypt(&self.caesar_cipher, &self.to_encrypt)
            }
            CipherMessage::ContentChanged(keyword) => {
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
            CipherMessage::InputChanged(input) => {
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
