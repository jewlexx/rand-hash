use num_bigint::RandBigInt;
use rand::thread_rng;
use sha2::{Digest, Sha256};

fn main() {
    let mut rng = thread_rng();

    let num = rng.gen_biguint(128);
    let num_str = num.to_string();

    let mut hasher = Sha256::new();

    hasher.update(&num_str);

    let res = hasher.finalize();

    let hash = res;

    println!("{:x}", hash)
}
