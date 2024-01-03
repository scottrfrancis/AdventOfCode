/*** Day 6
 * 
 */

use std::collections::HashMap;
use std::str::Lines;


fn parse_input(lines: Lines) -> HashMap<&str,(&str, &str)> {
    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();

    for line in lines {
        let name = line.trim().split(" = ").nth(0).unwrap().trim();
        let neighbors = line.trim().split(" = ").nth(1).unwrap().trim();
        let left = neighbors[1..neighbors.len() - 1].split(", ").nth(0).unwrap().trim();
        let right = neighbors[1..neighbors.len() - 1].split(", ").nth(1).unwrap().trim();

        nodes.insert(name, (left, right));
    }  

    nodes
}

fn num_steps(input: &str) -> usize {
    let mut steps = 0;

    // parse input -- save the instructions
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().trim();

    lines.next();   // skip the blank lines
    // build the network
    let network = parse_input(lines);
    let start_node = Some("AAA");

    // run the instructions over network
    let mut curr_node = start_node.unwrap();
    while curr_node != "ZZZ" {
        for dir in instructions.chars() {
            print!("{} ", curr_node);
            steps += 1;
            // move to next node in network.  If we hit the end, break out of loop. 
            match dir {
                'L' => curr_node = network[curr_node].0,
                'R' => curr_node = network[curr_node].1,
                _ => panic!("Invalid direction"),
            }
            
            if curr_node == "ZZZ" {
                return steps;
            }
        }
    }

    steps
}

fn num_parallel_steps(input: &str) -> usize {
    let mut steps = 0;

    // parse input -- save the instructions
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().trim();

    lines.next();   // skip the blank lines
    // build the network
    let network = parse_input(lines);

    // find vector of start nodes -- "**A"
    // copy to current nodes vector

    // get keys from network
    let mut curr_nodes: Vec<&str> = network.keys()
            .filter(|s| s.ends_with("A"))
            .map(|s| *s).collect();

    // run the instructions over network for ALL the current nodes
    // until ALL current nodes are at "**Z"
    
    // do all curr_nodes point to "**Z"?
    while curr_nodes.iter().any(|s| !s.ends_with("Z")) {
        for dir in instructions.chars() {
            // print!("{:?} ", curr_nodes);
            if steps % 1000 == 0 {
                println!("{} steps", steps);
            }
            
            steps += 1;
            // move to next node in network.  If we hit the end, break out of loop. 
            match dir {
                'L' => {
                    for node in curr_nodes.iter_mut() {
                        *node = network[*node].0;
                    }
                },
                'R' => {
                    for node in curr_nodes.iter_mut() {
                        *node = network[*node].1;
                    }
                },
                _ => panic!("Invalid direction"),
            }
            
            if curr_nodes.iter().all(|s| s.ends_with("Z")) {
                return steps;
            }
        }
    }

    steps
}

fn main() {
    println!("Part 1");
    let input  = include_str!("../input.txt");
    let steps = num_steps(input);
    println!("Number of steps: {}", steps);

    println!("Part 2");
    let steps = num_parallel_steps(input);
    println!("Number of steps: {}", steps);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_paths() {
        let steps = num_parallel_steps(INPUT3);
        assert_eq!(steps, 6);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        let steps = num_steps(input);
        assert_eq!(steps, 19637);
    
    }

    #[test]
    fn test_sample() {
        assert_eq!(num_steps(INPUT1), 2);
        assert_eq!(num_steps(INPUT2), 6);
    }

    const INPUT3: &str =
        "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)";

    const INPUT1: &str =
"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const INPUT2: &str =
        "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)";
        
}

