use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Minute = u32;

type GuardId = u32;

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).expect("Couldn't get input file");

    let reader = BufReader::new(file);

    let mut lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();

    lines.sort();

    let mut records: Vec<RecordEntry> = vec![];
    for line in lines {
        let entry = parse_line(&line);
        records.push(entry);
    }

    let mut minutes_asleep: HashMap<GuardId, HashMap<Minute, u32>> = HashMap::new();
    let mut current_guard: GuardId = 0;
    let mut previous_minute: Minute = 0;
    for entry in records {
        match entry {
            RecordEntry::StartShift { guard_id } => {
                current_guard = guard_id;
                previous_minute = 0;
            }
            RecordEntry::FallSleep { minute } => {
                previous_minute = minute;
            }
            RecordEntry::WakeUp { minute } => {
                let mut counter = match minutes_asleep.get(&current_guard) {
                    Some(current_minutes_asleep) => current_minutes_asleep.clone(),
                    None => HashMap::new(),
                };

                for m in previous_minute..minute {
                    match counter.get(&m) {
                        Some(count) => counter.insert(m.clone(), count + 1),
                        None => counter.insert(m, 1),
                    };
                }

                minutes_asleep.insert(current_guard, counter);
            }
        }
    }

    let mut current_max: (GuardId, Minute, u32) = (0, 0, 0);
    for (guard_id, minutes_sleep) in minutes_asleep.iter() {
        let (max_minute, max_minute_count) = get_totals(minutes_sleep);

        if max_minute_count > current_max.2 {
            current_max = (guard_id.clone(), max_minute, max_minute_count);
        }
    }

    let (guard_id, max_minute, _) = current_max;

    println!("{}", guard_id * max_minute);
}

fn get_totals(minutes_sleep: &HashMap<u32, u32>) -> (Minute, u32) {
    let mut total_minutes = 0;
    let mut max_minute = 0;
    let mut max_minute_count = 0;

    for (minute, count) in minutes_sleep.iter() {
        total_minutes += count;
        if count.clone() > max_minute_count {
            max_minute = minute.clone();
            max_minute_count = count.clone();
        }
    }

    return (max_minute, max_minute_count);
}

#[derive(Debug)]
enum RecordEntry {
    StartShift { guard_id: GuardId },
    WakeUp { minute: Minute },
    FallSleep { minute: Minute },
}

fn parse_line(line: &str) -> RecordEntry {
    if line.contains("begins shift") {
        let guard_id = parse_guard_id(&line);

        return RecordEntry::StartShift { guard_id };
    }

    let minute = parse_minute(line);

    if line.contains("falls asleep") {
        return RecordEntry::FallSleep { minute };
    }

    if line.contains("wakes up") {
        return RecordEntry::WakeUp { minute };
    }

    panic!("Got a line that couldn't be parsed");
}

fn parse_guard_id(line: &str) -> GuardId {
    let start_index = "[0000-00-00 00:00] Guard #".len();

    let guard_id_str = match line.find(" begins shift") {
        Some(end_index) => &line[start_index..end_index],
        None => panic!("incorrect line"),
    };

    guard_id_str.parse().expect("guard_id was not an GuardId")
}

fn parse_minute(line: &str) -> Minute {
    let start_index = "[0000-00-00 00:".len();
    let end_index = start_index + 2;

    line[start_index..end_index]
        .parse()
        .expect("minute was not a Minute")
}
