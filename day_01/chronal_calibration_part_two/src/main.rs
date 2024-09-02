use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).expect("Couldn't get file input");

    let reader = BufReader::new(file);
    let lines: Vec<_> = reader.lines().collect();

    let mut seen: HashSet<i32> = HashSet::from([0]);
    let mut calibration = 0;
    for line_result in lines.iter().cycle() {
        if let Ok(line) = line_result {
            let current_calibration = parse_line(&line);
            calibration += current_calibration;
            if seen.contains(&calibration) {
                break;
            } else {
                seen.insert(calibration);
            }
        }
    }

    println!("{}", calibration);
}

fn parse_line(str: &str) -> i32 {
    let chars: Vec<char> = str.chars().collect();

    let mut signal = 1;
    if let Some('-') = chars.get(0) {
        signal = -1;
    }

    let number: i32 = String::from_iter(&chars[1..]).parse().unwrap();

    signal * number
}
