use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .collect();

    let instructions = parse_lines(&lines);
    let steps: HashSet<char> = instructions
        .iter()
        .flat_map(|instruction| vec![instruction.step, instruction.parent])
        .collect();
    let step_parents = get_step_parents(&steps, &instructions);
    let mut step_ancestors: HashMap<char, HashSet<char>> = steps
        .iter()
        .map(|step| (step.clone(), get_ancestors(step.clone(), &step_parents)))
        .collect();

    let mut workers = create_workers(1 + 4);
    let mut answer = String::new();

    let number_of_steps = steps.len();
    let mut seconds_elapsed = 0;
    let mut nodes_being_worked_on: HashSet<char> = HashSet::new();

    while answer.len() < number_of_steps {
        let root_nodes = get_root_nodes(&step_ancestors);

        let workers_to_assign =
            get_assign_workers(&mut workers, &root_nodes, &nodes_being_worked_on);

        // assign workers
        for (id, step) in workers_to_assign.iter() {
            workers.insert(
                *id,
                Worker {
                    work: Some(WorkerWork {
                        step: *step,
                        time_remaining: 60 + (*step as i32) - (b'A' as i32),
                    }),
                },
            );
            nodes_being_worked_on.insert(*step);
        }

        let mut complete_this_second: Vec<(i32, WorkerWork)> = vec![];

        for (id, worker) in workers.iter_mut() {
            match worker.work {
                Some(work) => {
                    let time_remaining = work.time_remaining - 1;
                    worker.work = Some(WorkerWork {
                        step: work.step,
                        time_remaining,
                    });
                    if time_remaining < 0 {
                        complete_this_second.push((id.clone(), work.clone()));
                    }
                }
                None => (),
            }
        }

        complete_this_second.sort_by(|(_, a), (_, b)| a.step.cmp(&b.step));
        for (_, work) in complete_this_second.iter() {
            answer.push(work.step);
        }

        print!("{:03}", seconds_elapsed);
        for (_, worker) in workers.iter() {
            match worker.work {
                Some(work) => print!(" {} ", work.step),
                None => print!(" - "),
            }
        }
        println!("{}", answer);

        for (id, work) in complete_this_second.iter() {
            workers.insert(*id, Worker { work: None });
            nodes_being_worked_on.remove(&work.step);
            step_ancestors.remove(&work.step);
            for (_, ancestors) in step_ancestors.iter_mut() {
                ancestors.remove(&work.step);
            }
        }

        seconds_elapsed += 1;
    }

    print!("{:03}", seconds_elapsed);
    for (_, worker) in workers.iter() {
        match worker.work {
            Some(work) => print!(" {} ", work.step),
            None => print!(" - "),
        }
    }
    println!("{}", answer);

    Ok(())
}

fn get_root_nodes(step_ancestors: &HashMap<char, HashSet<char>>) -> Vec<char> {
    let mut root_nodes: Vec<char> = step_ancestors
        .iter()
        .filter_map(|(step, ancestors)| match ancestors.len() {
            0 => Some(step.clone()),
            _ => None,
        })
        .collect();

    root_nodes.sort();

    root_nodes
}

struct Instruction {
    step: char,
    parent: char,
}

fn parse_lines(lines: &Vec<String>) -> Vec<Instruction> {
    let parent_i = "Step ".len();
    let step_i = "Step ? must be finished before step ".len();

    lines
        .iter()
        .filter_map(|line| {
            let chars: Vec<char> = line.chars().collect();
            match (chars.get(parent_i), chars.get(step_i)) {
                (Some(&parent), Some(&step)) => Some(Instruction { step, parent }),
                _ => None,
            }
        })
        .collect()
}

fn get_parents(step_id: char, instructions: &Vec<Instruction>) -> HashSet<char> {
    let mut result: HashSet<char> = HashSet::new();

    for instruction in instructions {
        if instruction.step == step_id {
            result.insert(instruction.parent);
        }
    }

    result
}

fn get_step_parents(
    steps: &HashSet<char>,
    instructions: &Vec<Instruction>,
) -> HashMap<char, HashSet<char>> {
    steps
        .iter()
        .map(|step| (step.clone(), get_parents(step.clone(), &instructions)))
        .collect()
}

fn get_ancestors(step: char, step_dependencies: &HashMap<char, HashSet<char>>) -> HashSet<char> {
    let mut all_dependencies: HashSet<char> = HashSet::new();

    if let Some(dependencies) = step_dependencies.get(&step) {
        all_dependencies.extend(dependencies);

        for &parent_step in dependencies {
            let parent_dependencies = get_ancestors(parent_step, step_dependencies);
            all_dependencies.extend(parent_dependencies);
        }
    }

    all_dependencies
}

#[derive(Debug, Clone, Copy)]
struct WorkerWork {
    step: char,
    time_remaining: i32,
}

#[derive(Debug, Clone, Copy)]
struct Worker {
    work: Option<WorkerWork>,
}

fn create_workers(len: i32) -> HashMap<i32, Worker> {
    (0..len).map(|id| (id, Worker { work: None })).collect()
}

fn get_assign_workers(
    workers: &mut HashMap<i32, Worker>,
    available_steps: &Vec<char>,
    steps_in_progress: &HashSet<char>,
) -> Vec<(i32, char)> {
    let mut assign_workers: Vec<(i32, char)> = vec![];
    let mut to_assign_steps = available_steps
        .iter()
        .filter(|step| !steps_in_progress.contains(*step));

    for (id, worker) in workers.iter_mut() {
        match worker.work {
            None => {
                if let Some(root_node) = to_assign_steps.next() {
                    assign_workers.push((id.clone(), root_node.clone()));
                }
            }
            _ => (),
        }
    }

    assign_workers
}
