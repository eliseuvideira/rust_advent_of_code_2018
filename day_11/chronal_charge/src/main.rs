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

        // for y in 0..300 {
        //     for x in 0..300 {
        //         let value = grid[y][x];

        //         if value == 0 {
        //             print!("  {:02} ", value);
        //         } else if value > 0 {
        //             print!(" +{:02} ", value);
        //         } else {
        //             print!(" -{:02} ", value.abs());
        //         }
        //     }
        //     println!();
        // }

        let mut max_coordinate = (i32::MIN, i32::MIN, i32::MIN);
        for y in 1..=297 {
            for x in 1..=297 {
                let total: i32 = (0..3)
                    .flat_map(|dy: i32| (0..3).map(move |dx: i32| (x - 1 + dx, y - 1 + dy)))
                    .map(|(x, y)| grid[y as usize][x as usize])
                    .sum();

                if total > max_coordinate.2 {
                    max_coordinate = (x, y, total);
                }
            }
        }

        println!("{},{}", max_coordinate.0, max_coordinate.1);
    }
}
