use rand::{distributions::Alphanumeric, Rng};

pub fn alnum(inc_up_bound: usize) -> Vec<u8> {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(rand::thread_rng().gen_range(1..=inc_up_bound))
        .collect()
}
