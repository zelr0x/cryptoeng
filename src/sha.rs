use sha2::{Sha256, Digest};

pub fn sha256(bytes: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(bytes);
    h.finalize().try_into().unwrap()
}
