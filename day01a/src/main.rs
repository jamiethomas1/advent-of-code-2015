use std::io::Read;

fn main() {
    let input = include_bytes!("../input.txt");
    // println!("{}", String::from_utf8_lossy(input));
    let mut floor = 0;
    input.bytes().for_each(|b| {
        match b {
            Ok(b'(') => floor += 1,
            Ok(b')') => floor -= 1,
            _ => return,
        }
    });

    println!("{floor}");
}
