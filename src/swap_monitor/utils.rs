use ethers::utils::keccak256;
use hex;
use hex::{decode, encode};
use num_bigint::BigUint;

pub fn unpad(input: &str) -> String {
    let input = input.trim_start_matches(|c| c == '0' || c == 'x');
    if input.len() % 2 != 0 {
        format!("0x0{}", input)
    } else {
        format!("0x{}", input)
    }
}

pub fn generate_function_signature(function_name: String) -> String {
    let hash = keccak256(function_name.as_bytes());
    format!("0x{}", encode(&hash[..4]))
}

pub fn hex_to_utf8(hex: &str) -> String {
    let hex = hex.trim_start_matches("0x");
    let bytes = decode(hex).expect("Failed to decode hex string");
    String::from_utf8_lossy(&bytes).to_string()
}

pub fn hex_to_decimal(hex: &str) -> u8 {
    u8::from_str_radix(&hex[2..], 16).expect("Failed to parse hexadecimal")
}

pub fn hex_to_bigint(hex: &str) -> BigUint {
    let hex = hex.trim_start_matches("0x");
    let bytes = decode(hex).expect("Failed to decode hex string");
    num_bigint::BigUint::from_bytes_be(&bytes)
}
