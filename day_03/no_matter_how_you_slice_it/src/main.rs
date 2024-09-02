use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).expect("Couldn't open input file");

    let reader = BufReader::new(file);

    let mut fabric: HashMap<(i32, i32), i32> = HashMap::new();
    for line_result in reader.lines() {
        if let Ok(line) = line_result {
            let claim = get_claim_from_line(&line);

            let start_x = claim.position.x;
            let end_x = claim.position.x + claim.rectangle.width;
            let start_y = claim.position.y;
            let end_y = claim.position.y + claim.rectangle.height;

            for i in start_x..end_x {
                for j in start_y..end_y {
                    if let Some(count) = fabric.get(&(i, j)) {
                        fabric.insert((i, j), count + 1);
                    } else {
                        fabric.insert((i, j), 1);
                    }
                }
            }
        }
    }

    let mut two_or_more_claims = 0;
    for (_, count) in fabric.iter() {
        if count.clone() > 1 {
            two_or_more_claims += 1;
        }
    }

    println!("{}", two_or_more_claims);
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Rectangle {
    height: i32,
    width: i32,
}

#[derive(Debug, Clone, Copy)]
struct Claim {
    claim_id: i32,
    position: Position,
    rectangle: Rectangle,
}

fn get_claim_from_line(line: &str) -> Claim {
    let (claim_pieces, position_pieces, rectangle_pieces) = get_line_pieces(line);

    Claim {
        claim_id: get_claim_id(&claim_pieces),
        position: get_position(&position_pieces),
        rectangle: get_rectangle(&rectangle_pieces),
    }
}

fn get_claim_id(pieces: &str) -> i32 {
    pieces[1..].to_string().parse().unwrap()
}

fn get_position(pieces: &str) -> Position {
    let chars: Vec<char> = pieces.chars().collect();

    let mut i = 0;

    let mut y_str = String::new();
    while i < chars.len() {
        match chars.get(i) {
            Some(',') => break,
            Some(c) => y_str.push(c.clone()),
            None => break,
        };

        i += 1;
    }
    let y: i32 = y_str.parse().unwrap();

    i += 1;

    let mut x_str = String::new();
    while i < chars.len() {
        match chars.get(i) {
            Some(c) => x_str.push(c.clone()),
            None => break,
        };

        i += 1;
    }
    let x: i32 = x_str.parse().unwrap();

    Position { x, y }
}

fn get_rectangle(pieces: &str) -> Rectangle {
    let mut height_str = String::new();
    let mut width_str = String::new();

    let chars: Vec<char> = pieces.chars().collect();

    let mut i = 0;
    while i < chars.len() {
        match chars.get(i) {
            Some('x') => break,
            Some(c) => height_str.push(c.clone()),
            None => break,
        };

        i += 1;
    }
    i += 1;

    while i < chars.len() {
        match chars.get(i) {
            Some(c) => width_str.push(c.clone()),
            None => break,
        };

        i += 1;
    }

    let height: i32 = height_str.parse().unwrap();
    let width: i32 = width_str.parse().unwrap();

    Rectangle { height, width }
}

fn get_line_pieces(line: &str) -> (String, String, String) {
    let mut claim_pieces = String::new();
    let mut position_pieces = String::new();
    let mut rectangle_pieces = String::new();

    let chars: Vec<char> = line.chars().collect();

    let mut i = 0;
    while i < chars.len() {
        match chars.get(i) {
            Some(' ') => break,
            Some(c) => claim_pieces.push(c.clone()),
            None => break,
        };

        i += 1;
    }
    i += 3;

    while i < chars.len() {
        match chars.get(i) {
            Some(':') => break,
            Some(c) => position_pieces.push(c.clone()),
            None => break,
        };

        i += 1;
    }
    i += 2;

    while i < chars.len() {
        match chars.get(i) {
            Some(c) => rectangle_pieces.push(c.clone()),
            None => break,
        };

        i += 1;
    }

    (claim_pieces, position_pieces, rectangle_pieces)
}
