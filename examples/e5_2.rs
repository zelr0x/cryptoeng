use cryptoeng::{hex2b, sha};

fn main() {
    let res = sha::sha256(&hex2b("48 65 6C 6C 6F 2C 20 77 6F 72 6C 64 2E 20 20 20").unwrap());
    println!("{}", hex::encode(&res));
}
