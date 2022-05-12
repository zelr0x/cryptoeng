use rand::{
    distributions::{uniform::Uniform, Alphanumeric, DistIter, Distribution},
    prelude::ThreadRng,
    Rng,
};

/// Returns an infinite iterator over a range of 0..`n`-1.
pub fn iter_range(n: u64) -> DistIter<Uniform<u64>, ThreadRng, u64> {
    Uniform::new(0, n).sample_iter(rand::thread_rng())
}

/// Returns x random alphanumeric bytes where x is in range 1..=`n`.
pub fn alnum(n: usize) -> Vec<u8> {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(rand::thread_rng().gen_range(1..=n))
        .collect()
}
