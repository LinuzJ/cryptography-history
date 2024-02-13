use std::io::{self, Write};

fn caesar_shift(plain_text: String, shift: u8) -> String {
    plain_text
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base: u8 = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let char_offset: u8 = c as u8 - base;
                let char_shifted: u8 = char_offset + shift;
                let cipher_char: u8 = (char_shifted % 26) + base;
                return cipher_char as char;
            } else {
                c
            }
        })
        .collect()
}

pub fn caesar_shift_demo() {
    println!("Welcome to Caesar Cipher - 1st century BC");

    print!("Write the plaintext you will be using:");
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
            let cipher_text = caesar_shift(plain_text, shift);
            println!("{}", cipher_text)
        }
        Err(_) => {
            println!("idk");
        }
    }
}
