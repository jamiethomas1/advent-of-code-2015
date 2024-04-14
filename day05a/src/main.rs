use std::str;

fn main() {
    let input = include_bytes!("../input.txt");
    let lines = input.split(|c| *c == b'\n' )
        .map(|s| str::from_utf8(s).expect("Bad bytes in string"))
        .filter(|s| {
            no_bad_strings(&s) && over_three_vowels(&s) && has_double_letters(&s)
        });
    println!("{}", lines.count());
}

fn no_bad_strings(line: &str) -> bool {
    let bad_strings = ["ab", "cd", "pq", "xy"];
    for i in bad_strings {
        if line.contains(i) { return false }
    }
    true
}

fn over_three_vowels(line: &str) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut vowel_count = 0;
    line.chars().for_each(|c| {
        if vowels.contains(&c) { vowel_count += 1 }
    });
    vowel_count >= 3
}

fn has_double_letters(line: &str) -> bool {
    for i in 1..line.len() {
        if line.chars().nth(i).unwrap() == line.chars().nth(i-1).unwrap() { return true }
    }
    false
}
