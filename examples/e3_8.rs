use cryptoeng::{AES256, hex2b};

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
    let mut m: [u8; 16] = hex2b("53 9B 33 3B 39 70 6D 14 90 28 CF E1 D9 D4 A4 07")
        .unwrap()
        .try_into()
        .unwrap();
    let c = AES256::new(&k);
    c.dec_mut(&mut m);
    println!("{}", hex::encode(&m));
}

