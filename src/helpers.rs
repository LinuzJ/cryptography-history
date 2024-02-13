pub fn pretty_print_plain_to_cipher(plain_text: String, cipher_text: String) {
    println!("\n=================================================================");
    println!("| {: <28} | {: <28} |", "Plain Text", "Cipher Text");
    println!("| {: <28} | {: <28} |", plain_text, cipher_text);
    println!("=================================================================\n");
}
