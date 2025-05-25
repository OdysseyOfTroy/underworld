pub struct Caesar {
    shift: u8,
}

impl Caesar {
    pub fn new(shift: u8) -> Self {
        Self { shift: shift % 26 }
    }

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
