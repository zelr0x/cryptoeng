use cryptoeng::{random, sha::sha512n};
use std::{collections::HashMap, time::Instant};

struct CollisionData {
    original: Vec<u8>,
    collision: Vec<u8>,
    hash: Vec<u8>,
    attempts: usize,
}

fn main() {
    for n in (8..=48).step_by(8) {
        let before = Instant::now();
        let CollisionData {
            original,
            collision,
            hash,
            attempts,
        } = bday_sha512n(n);
        let elapsed = before.elapsed();
        println!(
            r#"SHA-512-{} collision found after {} attempts taking a total of {:.2?}
    collision:
        {}
    and
        {}
    hash to {}"#,
            n,
            attempts,
            elapsed,
            hex::encode(&original),
            hex::encode(&collision),
            hex::encode(&hash)
        );
    }
}

fn bday_sha512n(n: usize) -> CollisionData {
    let c_len = n; // 1..=64 byte long random byte seq
    let mut seen: HashMap<Vec<u8>, Vec<u8>> =
        HashMap::with_capacity(2u32.pow((n / 2).try_into().unwrap()) as usize);
    let mut attempts = 0;
    loop {
        attempts += 1;
        let c = random::alnum(c_len);
        let hash = sha512n(&c, n);
        match seen.get(&hash) {
            Some(original) => {
                if *original == c {
                    continue;
                }
                return CollisionData {
                    original: original.to_vec(),
                    collision: c,
                    hash: hash.to_vec(),
                    attempts,
                };
            }
            None => seen.insert(hash, c),
        };
    }
}
