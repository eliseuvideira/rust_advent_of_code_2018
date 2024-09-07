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

    let mut step_parents: HashMap<char, HashSet<char>> = HashMap::new();
    for &step_id in steps.iter() {
        let parents = get_parents(step_id, &instructions);
        step_parents.insert(step_id, parents);
    }

    let mut step_ancestors: HashMap<char, HashSet<char>> = HashMap::new();
    for &step in steps.iter() {
        let recursive_parents = get_ancestors(step, &step_parents);
        step_ancestors.insert(step, recursive_parents);
    }

    let mut answer = String::new();

    while step_ancestors.len() > 0 {
        let mut root_nodes: Vec<char> = step_ancestors
            .iter()
            .filter_map(|(step, ancestors)| match ancestors.len() {
                0 => Some(step.clone()),
                _ => None,
            })
            .collect();

        root_nodes.sort();

        if let Some(root_node) = root_nodes.get(0) {
            step_ancestors.remove(root_node);
            for (_, ancestors) in step_ancestors.iter_mut() {
                ancestors.remove(root_node);
            }

            answer.push(root_node.clone());
        }
    }

    println!("{}", answer);

    Ok(())
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
