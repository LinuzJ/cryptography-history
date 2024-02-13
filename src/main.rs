mod caesar_shift;
mod helpers;
mod monoalphapetic_cipher;

use caesar_shift::caesar_shift_demo;
use monoalphapetic_cipher::monoalphabetic_cipher_demo;
use std::io::{self, Write};

fn loopy(start: u32) {
    if start > 4 {
        return;
    }
    for cipher in start..5 {
        match cipher {
            1 => {
                caesar_shift_demo();
            }
            2 => {
                monoalphabetic_cipher_demo();
            }
            _ => println!("Not implemented yet"),
        }
    }
}

fn main() {
    println!("Welcome to a walkthrough of the different encryptions techniques used thought human history!");
    println!("Itinerary:");
    println!("1. Caesar Cipher - 1st century BC");
    println!("2. Mono-alphabetic Cipher - TODO");
    println!("3. Vigenère Cipher - 16th century");
    println!("4. Enigma - 1920-1940s");
    println!("5. Lucifer (DES) - 1974");
    println!("\nPress choose the number you want to start with OR press any other key to go in chronological order!");

    print!("Choose an option (0-4): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Parse input to u32
    match input.trim().parse::<u32>() {
        Ok(choice) => match choice {
            0 => {
                println!("Exiting the program. Goodbye!");
            }
            1..=5 => {
                loopy(choice);
            }
            _ => loopy(0),
        },
        Err(_) => {
            loopy(0);
        }
    }
}
