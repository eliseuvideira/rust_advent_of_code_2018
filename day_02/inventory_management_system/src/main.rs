use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).expect("Couldn't get file input");

    let reader = BufReader::new(file);

    let mut two_count = 0;
    let mut three_count = 0;
    for line_result in reader.lines() {
        if let Ok(line) = line_result {
            let (has_two, has_three) = parse_line(&line);
            if has_two {
                two_count += 1;
            }
            if has_three {
                three_count += 1;
            }
        }
    }

    let result = two_count * three_count;
    println!("{}", result);
}

fn parse_line(line: &str) -> (bool, bool) {
    let mut has_two = false;
    let mut has_three = false;

    let mut seen: HashMap<char, i32> = HashMap::new();
    for c in line.chars() {
        match seen.get(&c) {
            Some(count) => seen.insert(c, count + 1),
            None => seen.insert(c, 1),
        };
    }

    for (_, count) in seen.iter() {
        if count.clone() == 2 {
            has_two = true;
        } else if count.clone() == 3 {
            has_three = true;
        }

        if has_two && has_three {
            break;
        }
    }

    (has_two, has_three)
}
