use std::io::Read;

fn main() {
    let input = include_bytes!("../input.txt");
    let mut floor = 0;
    let mut steps = 0;
    input.bytes().for_each(|b| {
        if floor < 0 { return }
        match b {
            Ok(b'(') => { floor += 1; steps += 1 },
            Ok(b')') => { floor -= 1; steps += 1 }
            _ => return,
        }
    });

    println!("Floor: {floor}");
    println!("Steps: {steps}");
}
