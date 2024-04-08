use std::str;

use day02a::Present;

fn main() {
    let input = include_bytes!("../input.txt");
    let lines = input.split(|c| *c == b'\n').filter(|line| !line.is_empty());

    let mut presents: Vec<Present> = vec![];
    lines.for_each(|line| presents.push(Present::new(str::from_utf8(&line).expect("Expected valid bytes"))));

    let square_feet: u32 = presents.iter().fold(0, |n, present| n + present.wrapping_paper);
    println!("{square_feet}");
}
