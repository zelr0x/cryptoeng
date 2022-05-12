use cryptoeng::{hex2b, random, sha::sha512n};
use std::time::Instant;

fn main() {
    let tasks: Vec<(Vec<u8>, usize)> = vec![
        (hex2b("A9").unwrap(), 8),
        (hex2b("3D 4B").unwrap(), 16),
        (hex2b("3A 7F 27").unwrap(), 24),
        (hex2b("C3 C0 35 7C").unwrap(), 32),
    ];
    for (hash, n) in tasks {
        let before = Instant::now();
        let preimage = find_preimage(&hash, n);
        let elapsed = before.elapsed();
        println!(
            "Found pre-image for SHA-512-{} of {} in {:.2?}: {}",
            n,
            hex::encode(hash),
            elapsed,
            hex::encode(preimage),
        );
    }
}

fn find_preimage(hash: &[u8], n: usize) -> Vec<u8> {
    loop {
        let c = random::alnum(n);
        if &sha512n(&c, n) == hash {
            return c;
        }
    }
}
