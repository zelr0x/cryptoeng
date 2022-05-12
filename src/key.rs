use rand::{self, RngCore};

pub fn gen256() -> [u8; 256] {
    gen::<256>()
}

pub fn gen<const N: usize>() -> [u8; N] {
    let mut res = [0u8; N];
    rand::thread_rng().fill_bytes(&mut res);
    res
}
