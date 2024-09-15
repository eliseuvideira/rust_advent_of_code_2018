use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").expect("Failed to open input.txt");
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .collect();

    for line in lines {
        let grid_serial_number: i32 = line.parse().unwrap();

        let mut grid: Vec<Vec<i32>> = Vec::with_capacity(300);
        for y in 1..=300 {
            let mut row: Vec<i32> = Vec::with_capacity(300);
            for x in 1..=300 {
                let rack_id = x + 10;
                let power_level = rack_id * y;
                let power_level = power_level + grid_serial_number;
                let power_level = power_level * rack_id;
                let power_level = power_level / 100 % 10;
                let power_level = power_level - 5;

                row.push(power_level);
            }
            grid.push(row);
        }

        let mut max_coordinate = (0, 0, 0, i32::MIN);
        for s in 1..=300 {
            for y in 1..=(300 - s) {
                for x in 1..=(300 - s) {
                    let total = grid[(y - 1)..(y - 1) + s]
                        .iter()
                        .flat_map(|row| row[(x - 1)..((x - 1) + s)].iter())
                        .sum();

                    if total > max_coordinate.3 {
                        max_coordinate = (x, y, s, total);
                    }
                }
            }
        }

        println!(
            "{},{},{}",
            max_coordinate.0, max_coordinate.1, max_coordinate.2
        );
    }
}
