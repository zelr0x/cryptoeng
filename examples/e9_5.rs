use cryptoeng::random;

fn main() {
    let upper_bound = 128u64;
    let iter = random::iter_range(upper_bound);
    iter.take(upper_bound as usize * 2).for_each(|x| {
        assert!(x < upper_bound);
    });

    let upper_bound = u8::MAX as u64;
    let iter = random::iter_range(upper_bound);
    let collect = iter.take(100).collect::<Vec<u64>>();
    println!("{:?}", collect);
}
