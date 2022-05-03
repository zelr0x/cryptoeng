use sha2::{Digest, Sha256, Sha512};

pub fn sha256(bytes: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(bytes);
    h.finalize().try_into().unwrap()
}

pub fn sha512(bytes: &[u8]) -> [u8; 64] {
    let mut h = Sha512::new();
    h.update(bytes);
    h.finalize().try_into().unwrap()
}

/// Applies SHA512 and returns only N first bits.
///
/// Panics:
///   if N <= 0
///   if N > 64
pub fn sha512n<const N: usize>(bytes: &[u8]) -> [u8; N] {
    assert!(N > 0);
    assert!(N <= 64);

    sha512(bytes)[..N].try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn sha512n_1_works() {
        let expected = &sha512(b"hello sha512n")[..1];
        let got = sha512n::<1>(b"hello sha512n");
        assert_eq!(got.len(), 1);
        assert_eq!(expected, got);
    }

    #[test]
    fn sha512n_2_works() {
        let expected = &sha512(b"hello sha512n")[..2];
        let got = sha512n::<2>(b"hello sha512n");
        assert_eq!(got.len(), 2);
        assert_eq!(expected, got);
    }

    #[test]
    fn sha512n_64_works() {
        let expected = &sha512(b"hello sha512n")[..];
        let got = sha512n::<64>(b"hello sha512n");
        assert_eq!(got.len(), 64);
        assert_eq!(expected, got);
    }

    #[test]
    #[should_panic]
    fn sha512n_0_panics() {
        sha512n::<0>(b"hello sha512n");
    }

    #[test]
    #[should_panic]
    fn sha512n_65_panics() {
        sha512n::<65>(b"hello sha512n");
    }
}
