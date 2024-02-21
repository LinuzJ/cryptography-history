use std::{
    collections::HashMap,
    io::{self, Write},
};

use rand::seq::SliceRandom;

use crate::helpers::pretty_print_plain_to_cipher;

fn random_monoalphabetic_key() -> HashMap<char, char> {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz"
        .to_uppercase()
        .chars()
        .collect();
    let mut shuffle_alphabet: Vec<char> = alphabet.to_vec();
    let mut rng = rand::thread_rng();
    shuffle_alphabet.shuffle(&mut rng);

    let mut key = HashMap::new();
    for c in 0..alphabet.len() {
        key.insert(alphabet[c], shuffle_alphabet[c]);
    }
    return key;
}

fn monoalphabetic_cipher(plain_text: &str, key: HashMap<char, char>) -> String {
    plain_text
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                return key[&c];
            } else {
                c
            }
        })
        .collect()
}

pub fn monoalphabetic_cipher_demo() {
    println!("Welcome to Mono-alphabetic Cipher - TODO");

    print!("Write the plaintext you will be using: ");
    io::stdout().flush().unwrap();

    let mut plain_text = String::new();
    io::stdin()
        .read_line(&mut plain_text)
        .expect("Failed to read line");

    let random_key = random_monoalphabetic_key();
    let cipher_text = monoalphabetic_cipher(&plain_text.to_uppercase(), random_key);
    pretty_print_plain_to_cipher(plain_text, cipher_text);
}
