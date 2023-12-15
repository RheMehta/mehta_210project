// src/main.rs

use std::io::{ BufRead, BufReader, Error};
use std::{collections::{HashMap, VecDeque}, fs::File, io::{self, Write}};
use crate::bfs::bfs;

mod bfs; // Import the BFS module

fn read_file(path: &str) -> Result<(usize, HashMap<usize, Vec<usize>>), Error> {
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);
    let mut graph = HashMap::new();
    let mut vertices = 0;

    for (line_number, line) in buf_reader.lines().enumerate() {
        let line_str = match line {
            Ok(str) => str,
            Err(e) => {
                eprintln!("Error reading line {}: {:?}", line_number + 1, e);
                continue; // Skip this line and continue with the next one
            }
        };

        let v: Vec<usize> = line_str
            .trim()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if v.is_empty() {
            continue; // Skip empty lines
        }

        if v.len() == 1 {
            vertices = v[0];
        } else if v.len() == 2 {
            let label = v[0];
            let edge = v[1];

            graph
                .entry(label)
                .or_insert_with(Vec::new)
                .push(edge);

            graph
                .entry(edge)
                .or_insert_with(Vec::new)
                .push(label);
        } else {
            eprintln!("Invalid line format on line {}: {:?}", line_number + 1, v);
        }
    }

    Ok((vertices, graph))
}

fn main() {
    let results = match read_file("Epinions.txt") {
        Ok((_vertices, graph)) => (_vertices, graph),
        Err(e) => panic!("cannot be zero"),
    };

    if results.1.is_empty(){
        eprintln!("Error: graph is empty");
        return;
    }

    let start_node: usize = loop{
        print!("Enter start node: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                eprintln!("Invalid input. Please enter a valid nose number.");
            }
        }
    };

    let end_node: usize = loop {
        print!("Enter the end node: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                eprintln!("Invalid input. Please enter a valid node number.")
            }
        }
    };

    match bfs::bfs(&results.1 , start_node, end_node) {
        Some(distances) => {
            println!("Distance between node {} and node {} is {}", start_node, end_node , distances.len()-1);

        }

        None => {
            eprintln!("No path found from node {} to node {}." , start_node, end_node);
        }
    } 
}       

#[cfg(test)]
mod tests {
    use super::*;
    use std::{path::Path, result};

    #[test]
    fn test_read_file_valid_input() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("Epinions.txt");
        let expected_graph = {
            let mut map = HashMap::new();
            map.insert(1, vec![2]);
            map.insert(2, vec![1, 3]);
            map.insert(3, vec![2]);
            map
        };

        let result: Result<(usize, HashMap<usize, Vec<usize>>), Error> = read_file(path.to_str().unwrap());

        assert_eq!(result.is_ok(), true);

        if let Ok((vertices, graph)) = result {
            assert_eq!(vertices, 3);
            assert_eq!(graph, expected_graph);
        } else {
            panic!("Unexpected error");
        }
    }
    /// .
    ///
    /// # Panics
    ///
    /// Panics if .
    fn test_read_file_invalid_input() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("invalid_input.txt");
        assert!(read_file(path.to_str().unwrap()).is_err());
    }

    #[test]
    fn test_bfs_valid_path() {
        let mut graph = HashMap::new();
        graph.insert(1, vec![2, 3]);
        graph.insert(2, vec![1, 4]);
        graph.insert(3, vec![1]);
        graph.insert(4, vec![2]);
        assert_eq!(bfs(&graph, 1, 4), Some(vec![1, 2, 4]));
    }

    #[test]
    fn test_bfs_no_path() {
        let mut graph = HashMap::new();
        graph.insert(1, vec![2, 3]);
        graph.insert(2, vec![1]);
        graph.insert(3, vec![1]);
        assert_eq!(bfs(&graph, 1, 4), None);
    }
}

