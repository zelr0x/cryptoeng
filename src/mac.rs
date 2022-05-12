use cbc_mac::{CbcMac, Mac, digest::MacError};
use aes::{Aes256, cipher::InvalidLength};

type CbcMacAes256 = CbcMac<Aes256>;
pub struct CbcMacAES256(CbcMacAes256);

impl CbcMacAES256 {
    pub fn new(key: &[u8]) -> Result<CbcMacAES256, InvalidLength> {
        CbcMacAes256::new_from_slice(key)
            .map(|mac| CbcMacAES256(mac))
    }

    pub fn mac(&mut self, msg: &[u8]) -> Vec<u8> {
        let mac = &mut self.0;
        mac.update(msg);
        mac.finalize_reset().into_bytes().to_vec()
    }
    
    pub fn verify_slice(self, msg: &[u8], expected: &[u8]) -> Result<(), MacError> {
        let mut mac = self.0;
        mac.update(msg);
        mac.verify_slice(expected)
    }
}
