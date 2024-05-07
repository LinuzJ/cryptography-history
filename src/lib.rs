pub mod caesar_shift;
pub mod enigma;
pub mod monoalphabetic_cipher;
pub mod vigenere_cipher;

pub fn pretty_print_plain_to_cipher(plain_text: String, cipher_text: String) {
    println!("\n=================================================================");
    println!("| {: <28} | {: <28} |", "Plain Text", "Cipher Text");
    println!("| {: <28} | {: <28} |", plain_text, cipher_text);
    println!("=================================================================\n");
}

trait IndexForChar {
    fn index(&self) -> usize;
}

impl IndexForChar for char {
    fn index(&self) -> usize {
        *self as usize - 65
    }
}

trait ToChar {
    fn to_char(&self) -> char;
}

impl ToChar for usize {
    fn to_char(&self) -> char {
        ((*self % 26) as u8 + 65) as char
    }
}

/// Enigma rotor and reflector information from Wikipedia:
/// https://en.wikipedia.org/wiki/Enigma_rotor_details

pub const ROTORS: &[&str; 8] = &[
    "EKMFLGDQVZNTOWYHXUSPAIBRCJ",
    "AJDKSIRUXBLHWTMCQGZNPYFVOE",
    "BDFHJLCPRTXVZNYEIWGAKMUSQO",
    "ESOVPZJAYQUIRHXLNFTGKDCMWB",
    "VZBRGITYUPSDNHLXAWMJQOFECK",
    "JPGVOUMFYQBENHZRDKASXLICTW",
    "NZJHGRCXMYSWBOUFAIVLPEKQDT",
    "FKQHTLXOCBJSPDZRAMEWNIUYGV",
];

pub const NOTCHES: &[&str; 8] = &["Q", "E", "V", "J", "Z", "ZM", "ZM", "ZM"];

pub const REFLECTORS: &[&str; 3] = &[
    "EJMZALYXVBWFCRQUONTSPIKHGD",
    "YRUHQSLDPXNGOKMIEBFZCWVJAT",
    "FVPJIAOYEDRZXWGCTKUQSBNMHL",
];

pub const MAX_PLUGS: usize = 10;
