use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).expect("Couldn't get file input");

    let reader = BufReader::new(file);

    let lines: Vec<_> = reader.lines().map(|line| line.unwrap()).collect();

    let mut i = 0;
    while i < lines.len() {
        let current_line = lines.get(i).unwrap();
        let mut j = i + 1;

        while j < lines.len() {
            let next_line = lines.get(j).unwrap();

            if let Some(result) =
                get_difference_by_one(current_line.to_string(), next_line.to_string())
            {
                println!("{}", result);
                break;
            }

            j += 1;
        }

        i += 1;
    }
}

fn get_difference_by_one(current_line: String, next_line: String) -> Option<String> {
    let mut differences = 0;
    let mut same_letters: Vec<char> = vec![];

    let mut current_chars = current_line.chars();
    let mut next_chars = next_line.chars();
    loop {
        match (current_chars.next(), next_chars.next()) {
            (Some(a), Some(b)) => {
                if a != b {
                    differences += 1;
                } else {
                    same_letters.push(a);
                }
            }

            _ => break,
        }
    }

    match differences {
        1 => Some(String::from_iter(same_letters)),
        _ => None,
    }
}
