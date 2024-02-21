use std::io::{self, Write};

use crate::helpers::pretty_print_plain_to_cipher;

fn caesar_shift(plain_text: &str, shift: u8) -> String {
    plain_text
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let offset = shift % 26;
                let cipher_char: u8 = c as u8 + offset;
                return cipher_char as char;
            } else {
                c
            }
        })
        .collect()
}

pub fn caesar_shift_demo() {
    println!("Welcome to Caesar Cipher - 1st century BC");

    print!("Write the plaintext you will be using: ");
    io::stdout().flush().unwrap();

    let mut plain_text = String::new();
    io::stdin()
        .read_line(&mut plain_text)
        .expect("Failed to read line");

    print!("Enter the shift:");
    io::stdout().flush().unwrap();

    let mut shift_input = String::new();
    io::stdin()
        .read_line(&mut shift_input)
        .expect("Failed to read line");
    // Parse input to u32
    match shift_input.trim().parse::<u8>() {
        Ok(shift) => {
            let cipher_text = caesar_shift(&plain_text.to_uppercase(), shift);
            pretty_print_plain_to_cipher(plain_text, cipher_text);
        }
        Err(_) => {
            println!("idk");
        }
    }
}
