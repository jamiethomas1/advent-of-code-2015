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

    let mut santa = vec![Coord { x: 0, y: 0 }];
    let mut robo_santa = vec![Coord { x: 0, y: 0 }];

    for (i, el) in input.iter().enumerate() {
        let courier = if i % 2 == 0 { &mut santa } else { &mut robo_santa };

        match el {
            b'^' => courier.push(Coord { x: courier.last().unwrap().x, y: courier.last().unwrap().y + 1 }),
            b'>' => courier.push(Coord { x: courier.last().unwrap().x + 1, y: courier.last().unwrap().y }),
            b'v' => courier.push(Coord { x: courier.last().unwrap().x, y: courier.last().unwrap().y - 1 }),
            b'<' => courier.push(Coord { x: courier.last().unwrap().x - 1, y: courier.last().unwrap().y }),
            _ => continue
        }
    }

    let mut deliveries: Vec<Coord> = vec![];
    deliveries.append(&mut santa);
    deliveries.append(&mut robo_santa);
    deliveries.sort_by(|a, b| a.partial_cmp(b).expect("Not enough values to compare"));
    deliveries.dedup_by(|a, b| a == b);
    println!("Houses: {}", deliveries.len());
}
