use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();

    for line in lines {
        let instructions: Vec<u8> = line
            .split(' ')
            .filter_map(|entry| entry.parse::<u8>().ok())
            .collect();

        let root_node = build_node(&instructions);

        let sum = sum_metadata(&root_node);

        println!("{}", sum);
    }

    Ok(())
}

fn build_node_recurse(instructions: &Vec<u8>, start_index: usize, id: u8) -> (usize, Node) {
    let child_nodes_len = instructions.get(start_index).expect("child nodes len");
    let metadata_entries_len = instructions
        .get(start_index + 1)
        .expect("metadata entries len");

    let mut child_nodes: Vec<Node> = vec![];
    let mut metadata_entries: Vec<u8> = vec![];

    let mut index = start_index + 2;
    for c in 0..*child_nodes_len {
        let (current_index, node) = build_node_recurse(instructions, index, id + c + 1);

        child_nodes.push(node);
        index = current_index;
    }

    for _ in 0..*metadata_entries_len {
        if let Some(metadata_entry) = instructions.get(index) {
            metadata_entries.push(*metadata_entry);
        }

        index += 1;
    }

    (
        index,
        Node {
            id,
            child_nodes,
            metadata_entries,
        },
    )
}

fn build_node(instructions: &Vec<u8>) -> Node {
    let (_, node) = build_node_recurse(instructions, 0, 0);

    node
}

fn sum_metadata(node: &Node) -> u32 {
    let mut current_metadata: u32 = node.metadata_entries.iter().map(|v| *v as u32).sum();

    for child in node.child_nodes.iter() {
        current_metadata += sum_metadata(child);
    }

    current_metadata
}

struct Node {
    id: u8,
    child_nodes: Vec<Node>,
    metadata_entries: Vec<u8>,
}
