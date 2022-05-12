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

/// Applies SHA512 and returns only N first bytes.
///
/// Panics:
///   if N <= 0
///   if N > 64
pub fn sha512n_bytes_const<const N: usize>(bytes: &[u8]) -> [u8; N] {
    assert!(N > 0);
    assert!(N <= 64);

    sha512(bytes)[..N].try_into().unwrap()
}

pub fn sha512n_bytes(bytes: &[u8], n: usize) -> Vec<u8> {
    assert!(n > 0);
    assert!(n <= 64);

    sha512(bytes)[..n].try_into().unwrap()
}

pub fn sha512n(bytes: &[u8], n: usize) -> Vec<u8> {
    assert!(n > 0);
    assert!(n <= 512);

    hsbits(&sha512(bytes), n)
}

fn hsbits(bytes: &[u8], n: usize) -> Vec<u8> {
    let whole_bytes_n = n / 8;
    let bits = n % 8;
    let whole_bytes = &bytes[..whole_bytes_n];
    if bits == 0 || whole_bytes_n >= bytes.len() {
        whole_bytes.to_owned()
    } else {
        let mut res = Vec::with_capacity(whole_bytes_n + 1);
        res.extend_from_slice(&whole_bytes);
        let partial_byte = bytes[whole_bytes_n];
        res.push(byte_hsbits(partial_byte, bits));
        res
    }
}

/// Isolates n highest significance bits of a given byte.
///
/// Panics:
///   if `n` > 8
fn byte_hsbits(b: u8, n: usize) -> u8 {
    assert!(n <= 8);
    b >> (8 - n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_hsbits_255_works() {
        let expected = 63;
        let got = byte_hsbits(0xff, 6);
        assert_eq!(expected, got);
    }

    #[test]
    fn hsbits_works() {
        let src = vec![0x13, 0x37, 0xff, 0xff];
        let expected = vec![src[0], src[1], src[2], 127];
        let got = hsbits(&src, (src.len() - 1) * 8 + 7);
        assert_eq!(expected, got);
    }

    #[test]
    fn sha512n_bytes_const_1_works() {
        let expected = &sha512(b"hello sha512n")[..1];
        let got = sha512n_bytes_const::<1>(b"hello sha512n");
        assert_eq!(got.len(), 1);
        assert_eq!(expected, got);
    }

    #[test]
    fn sha512n_bytes_const_2_works() {
        let expected = &sha512(b"hello sha512n")[..2];
        let got = sha512n_bytes_const::<2>(b"hello sha512n");
        assert_eq!(got.len(), 2);
        assert_eq!(expected, got);
    }

    #[test]
    fn sha512n_bytes_const_64_works() {
        let expected = &sha512(b"hello sha512n")[..];
        let got = sha512n_bytes_const::<64>(b"hello sha512n");
        assert_eq!(got.len(), 64);
        assert_eq!(expected, got);
    }

    #[test]
    #[should_panic]
    fn sha512n_bytes_const_0_panics() {
        sha512n_bytes_const::<0>(b"hello sha512n");
    }

    #[test]
    #[should_panic]
    fn sha512n_bytes_const_65_panics() {
        sha512n_bytes_const::<65>(b"hello sha512n");
    }
}
