pub mod aes;
pub mod random;
pub mod sha;

use hex::{FromHex, FromHexError};

pub fn hex2b(s: &str) -> Result<Vec<u8>, FromHexError> {
    Vec::from_hex(s.split_whitespace().collect::<String>())
}
