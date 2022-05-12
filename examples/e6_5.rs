use cryptoeng::{mac::CbcMacAES256, hex2b};

fn main() {
    let key = hex2b(
        "80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
         00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01").unwrap();
    let msg = hex2b(
        "4D 41 43 73 20 61 72 65 20 76 65 72 79 20 75 73
         65 66 75 6C 20 69 6E 20 63 72 79 70 74 6F 67 72
         61 70 68 79 21 20 20 20 20 20 20 20 20 20 20 20").unwrap();
    let mut mac = CbcMacAES256::new(&key).unwrap();
    println!("{}", hex::encode(&mac.mac(&msg)));
}
