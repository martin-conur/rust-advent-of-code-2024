use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashMap, HashSet, VecDeque};


pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Transforms space separated numbers into a vector.
pub fn read_line_as_vec<P>(filename: P) -> Vec<i32> 
    where
        P: AsRef<Path>
{
    let mut result_vec = vec![];
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            result_vec =  line.split_whitespace().map(|v| v.parse::<i32>().unwrap()).collect()
        }
    }

    result_vec
}

pub fn topological_sort<'a>(pairs: &Vec<(i32, i32)>) -> Result<Vec<i32>, &'a str> {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut in_degree: HashMap<i32, usize> = HashMap::new();
    let mut nodes: HashSet<i32> = HashSet::new();

    // Build the graph and in-degree map
    for &(from, to) in pairs {
        graph.entry(from).or_insert_with(Vec::new).push(to);
        *in_degree.entry(to).or_insert(0) += 1;
        in_degree.entry(from).or_insert(0); // Ensure `from` node is in the map
        nodes.insert(from);
        nodes.insert(to);
    }

    // Initialize the queue with nodes having zero in-degree
    let mut queue: VecDeque<i32> = in_degree
        .iter()
        .filter(|&(_, &deg)| deg == 0)
        .map(|(&node, _)| node)
        .collect();

    let mut sorted_order = Vec::new();

    while let Some(node) = queue.pop_front() {
        sorted_order.push(node);
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if let Some(deg) = in_degree.get_mut(&neighbor) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    // Check for cycles
    if sorted_order.len() == nodes.len() {
        Ok(sorted_order)
    } else {
        Err("The graph contains a cycle")
    }
}