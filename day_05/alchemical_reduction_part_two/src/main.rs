use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).expect("Couldn't open file");

    let reader = BufReader::new(file);

    let polymer: String = reader.lines().filter_map(|line| line.ok()).collect();

    let (units, polymer_result) = chemical_reaction(&polymer);

    let mut min_polymer_after_removing_unit: (char, usize) = (' ', usize::MAX);
    for unit in units.iter() {
        let new_polymer: String = polymer_result
            .chars()
            .filter(|&c| !c.eq_ignore_ascii_case(unit))
            .collect();

        let (_, polymer_result) = chemical_reaction(&new_polymer);

        if polymer_result.len() < min_polymer_after_removing_unit.1 {
            min_polymer_after_removing_unit = (unit.clone(), polymer_result.len());
        }
    }

    let (_, min_polymer_len) = min_polymer_after_removing_unit;

    println!("{}", min_polymer_len);
}

fn chemical_reaction(polymer: &str) -> (HashSet<char>, String) {
    let mut units: HashSet<char> = HashSet::new();
    let mut result: Vec<_> = polymer.chars().collect();

    let mut should_loop_once_more = true;
    while should_loop_once_more {
        should_loop_once_more = false;

        let mut current_index = 0;
        while current_index < result.len() {
            let current_char = result.get(current_index);
            let next_char = result.get(current_index + 1);

            if let Some(c) = current_char {
                units.insert(c.clone().to_ascii_lowercase());
            };

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

    (units, result.iter().collect())
}
