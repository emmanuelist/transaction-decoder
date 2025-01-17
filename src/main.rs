use std::io;

fn  main() {
    println!("Welecome to the Transaction Decoder!");

    println!("Enter a transaction hex: ");
    let mut transaction_hex = String::new();
    io::stdin()
        .read_line(&mut transaction_hex)
        .expect("Failed to read line");

    println!("You entered: {}", transaction_hex.trim());
}