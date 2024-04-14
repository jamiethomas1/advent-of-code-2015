use std::str;

fn main() {
    let input = include_bytes!("../input.txt");

    let key: &str = str::from_utf8(input).unwrap().trim();

    for i in 1.. {
        let digest = gen_digest(key, i);
        if is_coin(&digest) { 
            println!("Coin found: {:x} with number {}", digest, i);
            break;
        }
    }
}

fn gen_digest(key: &str, seed: u32) -> md5::Digest {
    let seed: &str = &seed.to_string();
    md5::compute(key.to_owned() + seed)
}

fn is_coin(hash: &md5::Digest) -> bool {
    let hash = format!("{:x}", hash);
    hash.starts_with("00000")
}
