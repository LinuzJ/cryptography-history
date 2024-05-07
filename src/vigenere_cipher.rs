use std::{
    collections::HashMap,
    io::{self, Write},
};

use crate::helpers::pretty_print_plain_to_cipher;

pub fn vigenere_cipher_demo() {
    println!("\nWelcome to Vigenére Cipher - 16th Century");

    print!("Write the plaintext you will be using: ");
    io::stdout().flush().unwrap();

    let mut plain_text = String::new();
    io::stdin()
        .read_line(&mut plain_text)
        .expect("Failed to read line");
    remove_whitespace(&mut plain_text);

    print!("Enter the keyword for the cipher: ");
    io::stdout().flush().unwrap();

    let mut key = String::new();
    io::stdin()
        .read_line(&mut key)
        .expect("Failed to read line");
    remove_whitespace(&mut key);

    let cipher_text = vigenere_cipher(&plain_text.to_uppercase(), &key.to_uppercase());
    pretty_print_plain_to_cipher(plain_text.to_uppercase(), cipher_text);
}

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

fn vigenere_cipher(plain_text: &str, key: &str) -> String {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz"
        .to_uppercase()
        .chars()
        .collect();
    let key_map: HashMap<char, Vec<char>> = generate_vigenere_key_map();
    let keyword_as_vec: Vec<char> = key.chars().collect();
    let mut cipher_text: Vec<&char> = Vec::new();

    for (i, c) in plain_text.chars().enumerate() {
        if c == '\n' {
            continue;
        }
        // Get char to use from keyword
        let key_char: char = keyword_as_vec[i % (keyword_as_vec.len() - 1)];

        // Find which column to use
        let mut col = 0;
        while col < alphabet.len() && alphabet[col] != key_char {
            col += 1;
        }

        // Get cipher from key_map
        if key_map.contains_key(&key_char) {
            let cipher_char = &key_map[&c][col];
            cipher_text.push(cipher_char);
        }
    }

    return String::from_iter(cipher_text);
}

fn shift_text(unshifted: Vec<char>, shift: u8) -> Vec<char> {
    let start_of_alphabeth: u8 = 'A' as u8;
    let shifted = unshifted
        .iter()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let location_of_current = *c as u8 - start_of_alphabeth;
                let new_location_in_alphabeth = (location_of_current + shift) % 26;
                let new_ascii = start_of_alphabeth + new_location_in_alphabeth;
                return new_ascii as char;
            } else {
                *c
            }
        })
        .collect();
    return shifted;
}

fn generate_vigenere_key_map() -> HashMap<char, Vec<char>> {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz"
        .to_uppercase()
        .chars()
        .collect();
    let mut key_map = HashMap::new();

    for (i, c) in alphabet.iter().enumerate() {
        key_map.insert(*c, shift_text(alphabet.clone(), i as u8));
    }

    return key_map;
}
