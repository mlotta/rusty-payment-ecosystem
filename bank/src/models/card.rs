//! Card domain entity

use serde::{Deserialize, Serialize};
use shared::sql_macros::struct_to_sql;
use shared::usecase::rds::GetFieldsAsParams;
use shared::QuerySet;
use uuid::Uuid;

#[cfg(test)]
use rand::{thread_rng, Rng};

/// Card
#[derive(Deserialize, Serialize)]
#[struct_to_sql]
pub struct Card {
    #[serde(default = "uuid::Uuid::new_v4")]
    uuid: Uuid,
    #[serde(default)]
    pan: String,
    #[serde(default)]
    customer_uuid: Uuid,
    //TODO
    // #[serde(default)]
    // created_at:
}

pub type Pan = String;

/// Read the Major Industry Identifier associated with the PAN
pub fn get_pan_mii(pan: &Pan) -> &str {
    &pan[0..1]
}

/// Read the Account Identifier associated with the PAN
pub fn get_pan_account_identifier(pan: &Pan) -> &str {
    &pan[4..15]
}

/// Read the Bank Identification Number associated with the PAN
pub fn get_pan_bin(pan: &Pan) -> &str {
    &pan[0..8]
}

#[cfg(test)]
/// Generate a random pan
pub fn generate_random_pan() -> Pan {
    let mut rng = thread_rng();

    // Generate a random 15-digit number (first 15 digits of PAN)
    let mut pan_digits: Vec<u8> = (0..15)
        .map(|_| rng.gen_range(0..10) as u8) // Random digits from 0 to 9
        .collect();

    // Calculate the checksum
    pan_digits.push(calculate_luhn_checksum(&pan_digits));

    // Convert the digits to a string and return it
    pan_digits
        .iter()
        .map(|&d| char::from_digit(d as u32, 10).unwrap())
        .collect()
}

/// Calculate the Luhn checksum for a sequence of digits
#[cfg(test)]
fn calculate_luhn_checksum(digits: &[u8]) -> u8 {
    let sum: u32 = digits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &digit)| {
            let mut digit = digit as u32;
            if i % 2 == 0 {
                // Double every second digit from the right
                digit *= 2;
                if digit > 9 {
                    digit -= 9; // Subtract 9 if the result is greater than 9
                }
            }
            digit
        })
        .sum();

    let mod_10 = sum % 10;
    if mod_10 == 0 {
        0
    } else {
        10 - mod_10 as u8
    }
}

/// Validate a PAN
#[cfg(test)]
fn is_valid_pan(pan: &str) -> bool {
    let digits: Vec<u8> = pan
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as u8))
        .collect();

    calculate_luhn_checksum(&digits[0..(digits.len() - 1)]) == digits[digits.len() - 1]
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_valid_pan() {
        let payload1: [u8; 10] = [1, 7, 8, 9, 3, 7, 2, 9, 9, 7];
        assert_eq!(calculate_luhn_checksum(&payload1), 4);
        let pan1 = "17893729974";
        assert!(is_valid_pan(pan1));

        let payload2: [u8; 15] = [4, 9, 0, 2, 4, 5, 5, 3, 9, 7, 8, 8, 8, 9, 4];
        assert_eq!(calculate_luhn_checksum(&payload2), 9);
        let pan2 = "4902455397888949";
        assert!(is_valid_pan(pan2));

        let payload3: [u8; 15] = [4, 9, 0, 2, 4, 5, 5, 3, 9, 7, 8, 8, 8, 9, 5]; // Slightly change the payload
        assert_ne!(calculate_luhn_checksum(&payload3), 9);
        let pan3 = "4902455397888948"; // Slightly change the validation digit
        assert!(!is_valid_pan(pan3));
    }

    #[test]
    fn test_generate_random_pan() {
        let pan: Pan = generate_random_pan();
        assert!(is_valid_pan(&pan));
    }

    #[test]
    fn test_pan_getters() {
        // GIVEN a valid PAN
        let pan = "4902455397888949".to_string();

        // WHEN we try to get the account number, the BIC and the MII associated
        // with the PAN
        let mii = get_pan_mii(&pan);
        let bin = get_pan_bin(&pan);
        let account_number = get_pan_account_identifier(&pan);

        // THEN we get matching results
        assert_eq!(mii, "4");
        assert_eq!(bin, "49024553");
        assert_eq!(account_number, "45539788894");
    }
}
