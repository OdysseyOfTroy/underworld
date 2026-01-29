use simple_user_input::get_input;
use crate::cipher::{caesar_cipher::Caesar, cipher_traits::CipherTraits, vigenere_cipher::Vigenere, Cipher};

pub mod cipher;

fn main() -> iced::Result {
    iced::run(Cipher::update, Cipher::view)
}

fn basic() {
  let input: String = get_input("give me a string to encrypt");
  
  let ceaser_cipher = Caesar::new(10);
  let vigenere_cipher = Vigenere::new("Noggle");

  let ceaser_encrypted: String = ceaser_cipher.encrypt(input.as_str());
  let vigenere_encrypted: String = vigenere_cipher.encrypt(input.as_str());

  println!("your string if ceaser encrypted is: {}", ceaser_encrypted);
  println!("your string if vigenere encrytped is: {}", vigenere_encrypted);
}

mod simple_user_input {
    use std::io;
    pub fn get_input(prompt: &str) -> String{
        println!("{}",prompt);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {},
            Err(_no_updates_is_fine) => {},
        }
        input.trim().to_string()
    }
}
