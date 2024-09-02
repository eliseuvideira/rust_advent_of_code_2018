use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).expect("Couldn't get file input");

    let reader = BufReader::new(file);

    let mut calibration = 0;
    for line_result in reader.lines() {
        if let Ok(line) = line_result {
            let current_calibration = parse_line(&line);
            calibration += current_calibration;
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
