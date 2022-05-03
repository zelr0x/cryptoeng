use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes256;

pub struct AES256 {
    cipher: Aes256,
}

impl AES256 {
    pub fn new(key: &[u8; 32]) -> AES256 {
        let k = GenericArray::from_slice(key);
        let cipher = Aes256::new(&k);
        AES256 { cipher }
    }

    pub fn enc_mut(&self, block: &mut [u8; 16]) {
        let mut block = GenericArray::from_mut_slice(block);
        self.cipher.encrypt_block(&mut block);
    }

    pub fn dec_mut(&self, block: &mut [u8; 16]) {
        let mut block = GenericArray::from_mut_slice(block);
        self.cipher.decrypt_block(&mut block);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hex2b;

    #[test]
    fn it_works() {
        let k = hex2b(
            "
            80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
            00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01
            ",
        )
        .unwrap()
        .try_into()
        .unwrap();
        let mut m: [u8; 16] = hex2b("53 9B 33 3B 39 70 6D 14 90 28 CF E1 D9 D4 A4 07")
            .unwrap()
            .try_into()
            .unwrap();
        let m_copy = m.clone();
        let c = AES256::new(&k);
        c.enc_mut(&mut m);
        assert!(&m_copy != &m);
        c.dec_mut(&mut m);
        assert_eq!(&m_copy, &m);
    }
}
