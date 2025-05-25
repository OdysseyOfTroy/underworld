pub trait CipherTraits {
    fn encrypt(&self, plain_text: &str) -> String;

    fn decrypt(&self, cipher_text: &str) -> String;

    fn validate_solution(&self, cipher_text: &str, guess: &str) -> bool {
        self.decrypt(cipher_text).eq_ignore_ascii_case(guess)
    }
}
