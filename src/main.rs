use std::io;

fn main() {
    println!("Welecome to the Transaction Decoder!");

    println!("Enter a transaction hex: ");
    let mut transaction_hex = String::new();
    io::stdin()
        .read_line(&mut transaction_hex)
        .expect("Failed to read line");

    let transaction_hex = transaction_hex.trim();

    match validate_hex(transaction_hex) {
        Ok(_) => {
            match hex_to_bytes(transaction_hex) {
                Ok(bytes) => {
                    println!("Transaction bytes: {:?}", bytes);
                    // Proceed with decoding the transaction
                }
                Err(err) => {
                    eprintln!("Error converting hex to bytes: {}", err);
                }
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }

    // println!("You entered: {}", transaction_hex.trim());
}

/// Validate iif the input string is a valid hexadecimal string.
///
/// # Arguments
/// * `hex` - A string slice that holds the hexadecimal string.
///
/// Returns
///  * `Ok(())` if the input string is a valid hexadecimal string.
/// * `Err(String)` if the input string is not a valid hexadecimal string.
fn validate_hex(hex: &str) -> Result<(), String> {
    if hex.is_empty() {
        return Err("Empty string".to_string());
    }

    if hex.len() % 2 != 0 {
        return Err("Hex string must have an even number of characters".to_string());
    }

    if !hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("Hex string must contain only valid hexadecimal characters".to_string());
    }

    Ok(())
}
