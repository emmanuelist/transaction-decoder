use std::io;

fn main() {
    println!("Welcome to the Transaction Decoder!");

    println!("Enter a transaction hex:");
    let mut transaction_hex = String::new();
    io::stdin()
        .read_line(&mut transaction_hex)
        .expect("Failed to read input");

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
}

/// Validates if the input string is a valid hexadecimal string.
fn validate_hex(hex: &str) -> Result<(), String> {
    if hex.is_empty() {
        return Err("Input cannot be empty.".to_string());
    }

    if hex.len() % 2 != 0 {
        return Err("Hex string must have an even number of characters.".to_string());
    }

    if !hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("Input contains non-hexadecimal characters.".to_string());
    }

    Ok(())
}

/// Converts a hexadecimal string to a vector of bytes.
///
/// # Arguments
/// * `hex` - A string slice that holds the hex string.
///
/// # Returns
/// * `Ok(Vec<u8>)` containing the byte array if the conversion succeeds.
/// * `Err(String)` with an error message if the conversion fails.
fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, String> {
    (0..hex.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&hex[i..i + 2], 16)
                .map_err(|_| format!("Invalid hex byte: {}", &hex[i..i + 2]))
        })
        .collect()
}
