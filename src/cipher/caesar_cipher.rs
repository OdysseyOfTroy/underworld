use iced::{widget::{button, column, Column}};

use crate::cipher::cipher_traits::CipherTraits;

#[derive(Default)]
pub struct Caesar {
    pub shift: u8,
}

#[derive(Debug, Clone)]
pub enum Message {
    Increment,
    Decrement
}


impl Caesar {
    pub fn view(&self) -> Column<'_, Message> {
        column![
            button("test").on_press(Message::Increment),
        ]
    
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.shift += 1;
            }
            Message::Decrement => {
                self.shift -= 1;
            }
        }
    }

    pub fn new(shift: u8) -> Self {
        Self { shift: shift % 26 }
    }

    fn shift_char(c: char, amount: u8) -> char {
        if c.is_ascii_uppercase() {
            (((c as u8 - b'A' + amount) % 26) + b'A') as char
        } else if c.is_ascii_lowercase() {
            (((c as u8 - b'a' + amount) % 26) + b'a') as char
        } else {
            c
        }
    }
}

impl CipherTraits for Caesar {
    fn encrypt(&self, plain_text: &str) -> String {
        plain_text
            .chars()
            .map(|c| Self::shift_char(c, self.shift))
            .collect()
    }

    fn decrypt(&self, encrpyted_text: &str) -> String {
        encrpyted_text
            .chars()
            .map(|c| Self::shift_char(c, 26 - self.shift))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn caesar_encrypts_correctly() {
        let c = Caesar { shift: 10 };
        let plain_text = "Secret Message";
        let expected = "Combod Wocckqo";
        let encrpyted = c.encrypt(plain_text);
        assert_eq!(encrpyted, expected);
    }


    #[test]
    fn caesar_upper_with_wrap_encrypts_correctly() {
        let c = Caesar { shift: 10 };
        let plain_text = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let expected = "KLMNOPQRSTUVWXYZABCDEFGHIJ";
        let encrpyted = c.encrypt(plain_text);
        assert_eq!(encrpyted, expected);
    }
    
    #[test]
    fn caesar_lower_with_wrap_encrypts_correctly() {
        let c = Caesar { shift: 10 };
        let plain_text = "abcdefghijklmnopqrstuvwxyz";
        let expected = "klmnopqrstuvwxyzabcdefghij";
        let encrpyted = c.encrypt(plain_text);
        assert_eq!(encrpyted, expected);
    }

    #[test]
    fn caesar_decrypts_correctly() {
        let c = Caesar { shift: 10 };
        let plain_text = "Secret Message";
        let encrypted = c.encrypt(plain_text);
        let decrypted = c.decrypt(&encrypted);
        assert_eq!(decrypted, plain_text);
    }
}
