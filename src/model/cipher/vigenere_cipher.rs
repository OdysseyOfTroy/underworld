use super::cipher_traits::CipherTraits;

#[derive(Debug, PartialEq)]
pub enum VigenereError {
    EmptyKey,
}

#[derive(Default, Debug)]
pub struct Vigenere {
    pub keyword: String,
    key: Vec<u8>,
}

impl Vigenere {
    pub fn new(keyword: impl Into<String>) -> Result<Self, VigenereError> {
        let keyword = keyword.into();

        let key: Vec<u8> = keyword
            .bytes()
            .filter(|c| c.is_ascii_alphabetic())
            .map(|c| c.to_ascii_uppercase() - b'A')
            .collect();

        if key.is_empty() {
            return Err(VigenereError::EmptyKey);
        }

        Ok(Self { keyword, key })
    }

    pub fn transform(&self, plain_text: &str, decrypt: bool) -> String {
        let mut result = String::new();
        let mut key_index = 0;

        for c in plain_text.chars() {
            if c.is_ascii_alphabetic() {
                let is_upper = c.is_uppercase();
                let base = if is_upper { b'A' } else { b'a' };
                let offset = c as u8 - base;
                let key = &self.key[key_index % self.key.len()];
                let shift = if decrypt {
                    (26 + offset - key) % 26
                } else {
                    (offset + key) % 26
                };
                result.push((base + shift) as char);
                key_index += 1;
            } else {
                result.push(c);
            }
        }

        result
    }
}

impl CipherTraits for Vigenere {
    fn encrypt(&self, plain_text: &str) -> String {
        self.transform(plain_text, false)
    }

    fn decrypt(&self, cipher_text: &str) -> String {
        self.transform(cipher_text, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_key_fails() {
        let err = Vigenere::new("").unwrap_err();
        assert_eq!(err, VigenereError::EmptyKey);
    }

    #[test]
    fn vigenere_encrypt_upper_key_encrypts_correctly() {
        let v = Vigenere::new("AAAAAAAAAAAAAAAAAAAAAAAAAA").unwrap();
        let plain = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let expected = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let encrypted = v.encrypt(plain);
        assert_eq!(expected, encrypted);
    }

    #[test]
    fn vigenere_encrypt_key_longer_than_message_encrypts_correctly() {
        let v = Vigenere::new("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        let plain = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let expected = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let encrypted = v.encrypt(plain);
        assert_eq!(expected, encrypted);
    }

    #[test]
    fn vigenere_encrypt_key_shorter_than_message_encrypts_correctly() {
        let v = Vigenere::new("b").unwrap();
        let plain = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let expected = "BCDEFGHIJKLMNOPQRSTUVWXYZA";
        let encrypted = v.encrypt(plain);
        assert_eq!(expected, encrypted);
    }

    #[test]
    fn vigenere_encrypt_decrypt() {
        let v = Vigenere::new("KEY").unwrap();
        let plain = "Attack at dawn!";
        let encrypted = v.encrypt(plain);
        let decrypted = v.decrypt(&encrypted);
        assert_eq!(decrypted, plain);
    }
}
