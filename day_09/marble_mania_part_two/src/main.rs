use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").expect("Missing input.txt file");
    let lines: Vec<(u32, u32)> = BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.replace(" players; last marble is worth ", ",")
                .replace(" points", "")
                .split(",")
                .map(String::from)
                .collect::<Vec<String>>()
        })
        .map(|result: Vec<String>| (result[0].parse().unwrap(), result[1].parse().unwrap()))
        .collect();

    for (num_players, last_marble) in lines {
        let mut players_score: Vec<u32> = vec![0; num_players as usize];
        let mut marble_board: VecDeque<u32> = VecDeque::with_capacity(last_marble as usize + 1);

        marble_board.push_back(0);

        for current_marble in 1..=last_marble {
            if current_marble % 23 == 0 {
                marble_board.rotate_left(7);
                let marble = marble_board.pop_front().unwrap();
                players_score[(current_marble % num_players) as usize] = players_score
                    [(current_marble % num_players) as usize]
                    + current_marble
                    + marble;
                marble_board.rotate_right(1);
            } else {
                marble_board.rotate_right(1);
                marble_board.push_back(current_marble);
                marble_board.rotate_right(1);
            }
        }

        let high_score = players_score.iter().max().unwrap().clone();
        println!("{}", high_score);
    }
}
