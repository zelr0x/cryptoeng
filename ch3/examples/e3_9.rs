use ch3::{AES256, hex2b};

fn main() {
    let k = hex2b(
            "
            80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
            00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01
            ",
        )
        .unwrap()
        .try_into()
        .unwrap();
    let mut m: [u8; 16] = hex2b("29 6C 93 FD F4 99 AA EB 41 94 BA BC 2E 63 56 1D")
        .unwrap()
        .try_into()
        .unwrap();
    let c = AES256::new(&k);
    c.enc_mut(&mut m);
    println!("{}", hex::encode(&m));
}

