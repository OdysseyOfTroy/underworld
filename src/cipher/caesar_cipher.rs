use crate::cipher::cipher_traits::CipherTraits;

pub struct Caesar {
    pub shift: i16,
}

impl Caesar {
    pub fn new(shift: i16) -> Self {
        Self {
            shift: shift.rem_euclid(26),
        }
    }

    fn shift_char(&self, c: char) -> char {
        if c.is_ascii_uppercase() {
            let base = b'A' as i16;
            let shifted = (c as i16 - base + self.shift).rem_euclid(26) + base;
            char::from_u32(shifted as u32).unwrap()
        } else if c.is_ascii_lowercase() {
            let base = b'a' as i16;
            let shifted = (c as i16 - base + self.shift).rem_euclid(26) + base;
            char::from_u32(shifted as u32).unwrap()
        } else {
            c
        }
    }
}

impl Default for Caesar {
    fn default() -> Self {
        Self { shift: 10 }
    }
}

impl CipherTraits for Caesar {
    fn encrypt(&self, plain_text: &str) -> String {
        plain_text.chars().map(|c| self.shift_char(c)).collect()
    }

    fn decrypt(&self, encrpyted_text: &str) -> String {
        let inverse = Caesar::new(-self.shift);
        encrpyted_text
            .chars()
            .map(|c| inverse.shift_char(c))
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
