use cryptoeng::key;

fn main() {
    let key = key::gen256();
    println!("{}", hex::encode(&key));
}
