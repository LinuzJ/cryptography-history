use clap::{Parser, Subcommand};
use cryptography_history::{
    caesar_shift::caesar_shift_demo, enigma::demo::enigma_demo,
    monoalphabetic_cipher::monoalphabetic_cipher_demo, vigenere_cipher::vigenere_cipher_demo,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Start {},
    #[command(arg_required_else_help = true)]
    StartOn {
        chapter: i32,
    },
    List {},
}

fn run_history(start: i32) {
    if start > 4 {
        return;
    }
    for cipher in start..7 {
        match cipher {
            1 => {
                caesar_shift_demo().unwrap();
            }
            2 => {
                monoalphabetic_cipher_demo();
            }
            3 => {
                vigenere_cipher_demo();
            }
            4 => {
                enigma_demo();
            }
            _ => println!("Not implemented yet"),
        }
    }
}

fn print_introduction() {
    println!("\n=================================================================");
    println!("Welcome to a walkthrough of the different encryptions techniques used thought human history!");
    println!("Itinerary:");
    println!("1. Caesar Cipher - 1st century BC");
    println!("2. Mono-alphabetic Cipher - TODO");
    println!("3. Vigenère Cipher - 16th century");
    println!("4. Enigma - 1920-1940s");
    println!("5. Lucifer (DES) - 1974");
    println!("6. Diffie-Hellman Algorithm - 1976");
    println!("7. RSA Algorithm - 1977");
    println!("=================================================================\n\n");
}

fn main() {
    let cli = Args::parse();

    match cli.command {
        Commands::Start {} => {
            print_introduction();
            run_history(1);
        }
        Commands::StartOn { chapter } => {
            if chapter < 0 || chapter > 7 {
                println!("Provide a valid chapter, please")
            }

            run_history(chapter);
        }
        Commands::List {} => print_introduction(),
    }
}
