#[derive(Debug, PartialOrd, PartialEq)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

fn main() {
    let input = include_bytes!("../input.txt");
    // Use a vector of tuples as co-ordinates. Always (0, 0) initial element. > becomes (1, 0) etc.
    // Add tuples for each instruction & then strip vector of duplicates.
    // For added understandability, use a co-ordinate struct instead of a tuple.

    let mut deliveries = vec![Coord { x: 0, y: 0 }];

    input.iter().for_each(|c| {
        match *c {
            b'^' => deliveries.push(Coord { x: deliveries.last().unwrap().x, y: deliveries.last().unwrap().y + 1 }),
            b'>' => deliveries.push(Coord { x: deliveries.last().unwrap().x + 1, y: deliveries.last().unwrap().y }),
            b'v' => deliveries.push(Coord { x: deliveries.last().unwrap().x, y: deliveries.last().unwrap().y - 1 }),
            b'<' => deliveries.push(Coord { x: deliveries.last().unwrap().x - 1, y: deliveries.last().unwrap().y }),
            _ => return
        }
    });

    deliveries.sort_by(|a, b| a.partial_cmp(b).expect("Not enough values to compare"));
    deliveries.dedup_by(|a, b| a == b);
    println!("{}", deliveries.len());
}
