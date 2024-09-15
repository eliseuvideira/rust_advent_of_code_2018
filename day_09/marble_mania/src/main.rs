use std::collections::HashMap;
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
        let mut players_score: HashMap<usize, u32> = vec![0; num_players as usize]
            .iter()
            .enumerate()
            .map(|(index, _)| (index, 0))
            .collect();
        let mut marble_board: Vec<u32> = Vec::with_capacity(last_marble as usize + 1);

        let mut round = 0;
        let mut current_player = 0;
        let mut current_index = 0;

        marble_board.push(0);
        // print!("[-----] ");
        // println!(" ({:05}) ", 0);

        round += 1;

        for current_marble in 1..=last_marble {
            let mut marble = current_marble;

            if current_marble % 23 == 0 {
                let player_score = players_score.get(&current_player).unwrap();

                let prize_marble_index = get_index_previous(current_index, 7, marble_board.len());

                let prize_marble = marble_board[prize_marble_index];

                players_score.insert(current_player, player_score + current_marble + prize_marble);

                marble_board.remove(prize_marble_index);

                current_index = prize_marble_index;
                marble = marble_board[current_index];
            } else {
                let next_index = get_index_next(current_index, 2, marble_board.len());
                current_index = next_index;

                marble_board.insert(current_index, current_marble);
            }

            // print!("[{:05}] ", current_player + 1);
            // for marble_item in marble_board.iter() {
            //     if marble_item.clone() == marble {
            //         print!(" ({:05}) ", marble_item.clone());
            //     } else {
            //         print!("  {:05}  ", marble_item.clone());
            //     }
            // }
            // println!();

            current_player = get_index_next(current_player, 1, players_score.len());
            round += 1;
        }

        if let Some(high_score) = players_score.iter().map(|(_, score)| score.clone()).max() {
            println!("High Score: {}", high_score);
        }
    }
}

fn get_index_previous(current_index: usize, offset: usize, max_len: usize) -> usize {
    let mut index = current_index;

    for _ in 0..offset {
        index = if index == 0 { max_len - 1 } else { index - 1 };
    }

    index
}

fn get_index_next(current_index: usize, offset: usize, max_len: usize) -> usize {
    let mut index = current_index;

    for _ in 0..offset {
        index = if index == max_len - 1 { 0 } else { index + 1 }
    }

    index
}
