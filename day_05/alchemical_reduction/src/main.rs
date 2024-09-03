use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).expect("Couldn't open file");

    let reader = BufReader::new(file);

    let polymer: String = reader.lines().filter_map(|line| line.ok()).collect();

    let result = chemical_reaction(&polymer);

    println!("{}", result.len());
}

fn chemical_reaction(polymer: &str) -> String {
    let mut result: Vec<_> = polymer.chars().collect();

    let mut should_loop_once_more = true;
    while should_loop_once_more {
        should_loop_once_more = false;

        let mut current_index = 0;
        while current_index < result.len() {
            let current_char = result.get(current_index);
            let next_char = result.get(current_index + 1);

            match (current_char, next_char) {
                (Some(current_char), Some(next_char)) => {
                    if current_char != next_char && current_char.eq_ignore_ascii_case(next_char) {
                        result.drain(current_index..=current_index + 1);
                        should_loop_once_more = true;
                    } else {
                        current_index += 1;
                    }
                }
                _ => current_index += 1,
            };
        }
    }

    result.iter().collect()
}
