/*** Day 12
 * 
 * Rectangular Distance - run pair-wise
 *  
 */


fn parse_line(line: &str) -> (Vec<char>, Vec<usize>) {
    let mut parts = line.split_whitespace();
    let map = parts.next().unwrap();
    let runs = parts.next().unwrap();
    let map_row: Vec<char> = map.chars().collect();
    let run_row: Vec<usize> = runs.split(',').map(|run| run.parse().unwrap()).collect();
    (map_row, run_row)
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Vec<usize>>) {
    let mut spring_map: Vec<Vec<char>> = Vec::new();
    let mut run_lengths: Vec<Vec<usize>> = Vec::new();

    for line in input.lines() {
        let (map_row, run_row) = parse_line(line);
        spring_map.push(map_row);
        run_lengths.push(run_row);
    }

    (spring_map, run_lengths)
}

fn compute_run_lengths(map_row: &[char]) -> Vec<usize> {
    let mut run_lengths: Vec<usize> = Vec::new();
    let mut run_count = 0;
    for &ch in map_row {
        run_count = match(ch, run_count) {
            ('#', _) => run_count + 1,
            ('.', 0 ) => 0,
            ('.', _) if run_count > 0 => {
                run_lengths.push(run_count);
                0
            },
            (_, _) => panic!("Invalid character in map row: {}", ch),
        }
    }

    if run_count > 0 {
        run_lengths.push(run_count);
    }

    run_lengths
}

fn is_good_line(map_row: &[char], run_row: &[usize]) -> bool {
    let run_lengths = compute_run_lengths(map_row);
    run_lengths == run_row
}

#[allow(dead_code)]
// make a memoized version of this function
fn memoize(f: fn(&[char], &[usize]) -> bool) -> impl FnMut(&[char], &[usize]) -> bool {
    let mut memo = std::collections::HashMap::new();
    move |map_row, run_row| {
        let key = (map_row.to_vec(), run_row.to_vec());
        if !memo.contains_key(&key) {
            memo.insert(key.clone(), f(map_row, run_row));
        }
        memo[&key]
    }
}

// get the index of the next '?' value in the array
fn next_unknown(row: &[char], idx: usize) -> usize {
    // row.iter().skip(idx).position(|&ch| ch == '?').unwrap_or(row.len())
    // let idx = row.len();        // aka flag for not found

    for i in idx..row.len() {
        if row[i] == '?' {
            return i;
        }
    }  

    row.len()
}

fn count_dfs_permutations(map_row: &[char], run_row: &[usize]) -> usize {
    struct DFS<'s> {
        map_row: &'s [char],
        perm_row: &'s mut [char],
        run_row: &'s [usize],
        count: usize,

        f: &'s dyn Fn(&mut DFS, usize) -> (),
    }
    let mut dfs = DFS {
        f: &|dfs:&mut DFS<'_>, idx: usize| -> () {
            // check if we're done
            let perm_unknowns = dfs.perm_row.iter().filter(|&ch| *ch == '?').count();
            if perm_unknowns == 0 {
                if is_good_line(&dfs.perm_row, dfs.run_row) {
                    dfs.count += 1;
                }
                return;
            }

            // make a choice, recurse
            dfs.perm_row[idx] = '#';
            (dfs.f)(dfs, next_unknown(dfs.map_row, idx + 1));
            dfs.perm_row[idx] = '.';
            (dfs.f)(dfs, next_unknown(dfs.map_row, idx + 1));
            // if we're here, we're done with this choice, backtrack
            dfs.perm_row[idx] = '?';
        },

        count: 0,
        perm_row: &mut map_row.to_vec(),
        run_row: run_row,
        map_row: map_row,
    };

    (dfs.f)(&mut dfs, next_unknown(&map_row, 0));

    dfs.count
}

fn unfold_row(text_row: &str) -> String {
    let (map_row, run_row) = parse_line(text_row);
    // make 5 copeis of the map row
    let mut unfolded_map = Vec::new();
    let mut unfolded_runs = Vec::new();
    for _i in 0..5 {
        unfolded_map.push(map_row.clone());
        unfolded_runs.push(run_row.clone());
    }
    
    // join the unfolded vectors to strings
    let unfolded_map_str: Vec<String> = unfolded_map.iter().map(|row| row.iter().collect()).collect();
    let unfolded_runs_str: Vec<String> = unfolded_runs.iter().map(|row| row.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(",")).collect();

    // join the two strings with a space
    let unfolded_map_str = unfolded_map_str.join("?");
    let unfolded_runs_str = unfolded_runs_str.join(",");
    let unfolded_row = unfolded_map_str + " " + &unfolded_runs_str;
    
    unfolded_row
}

fn sum_possible_arrangements(input: &str) -> usize {
    let (spring_map, run_lengths) = parse_input(input);
    let mut total = 0;
    // let mut validator = memoize(is_good_line);
    // let mut validator = is_good_line;

    for (map_row, run_row) in spring_map.iter().zip(run_lengths.iter()) {
        // total += count_successful_permutations(map_row, run_row, &mut validator);
        total += count_dfs_permutations(map_row, run_row);
    }

    total
}

fn main() {
    println!("Day 12");

    println!("Part 1");
    let input = include_str!("../input.txt");
    let result = sum_possible_arrangements(input);
    println!("{} \n", result);
    
    println!("Part 2");
    let mut unfolded_lines = Vec::new();
    for line in input.lines() {
        let unfolded_row = unfold_row(line);
        // append unfolded row to unfolded input       
        unfolded_lines.push(unfolded_row);
    }

    let unfolded_input = unfolded_lines.join("\n");
    let result = sum_possible_arrangements(&unfolded_input.as_str());    
    println!("{} \n", result);
}

#[cfg(test)]
mod tests {
    use std::vec;
    use super::*;

    const INPUT1: &str = 
        "???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1";

    #[test]
    fn test_next_unknown() {
        let row = vec!['#', '?', '.', '?', '#', '?', '?', '#'];
        let idx = next_unknown(&row, 0);
        assert_eq!(idx, 1);

        let idx = next_unknown(&row, idx + 1);
        assert_eq!(idx, 3);

        let row = vec!['?', '?', '#', '#', '?', '#', '#', '?'];
        let idx = next_unknown(&row, 0);
        assert_eq!(idx, 0);

        let idx = next_unknown(&row, idx + 1);
        assert_eq!(idx, 1);

        let idx = next_unknown(&row, idx + 1);
        assert_eq!(idx, 4);

        let idx = next_unknown(&row, idx + 1);
        assert_eq!(idx, 7);

        let row = vec!['#', '#', '#', '#', '#', '#', '#', '#'];
        let idx = next_unknown(&row, 0);
        assert_eq!(idx, 8);
    }   

    #[test]
    fn test_unfolded_sample() {
        let input = INPUT1;
        let mut unfolded_lines = Vec::new();
        for line in input.lines() {
            let unfolded_row = unfold_row(line);
            // append unfolded row to unfolded input       
            unfolded_lines.push(unfolded_row);
        }

        let unfolded_input = unfolded_lines.join("\n");
        let result = sum_possible_arrangements(&unfolded_input.as_str());
        assert_eq!(result, 525152);
    }

    #[test]
    fn test_unfold_line() {
        let text_row = ".# 1";
        let unfolded_row = unfold_row(text_row);
        assert_eq!(unfolded_row, ".#?.#?.#?.#?.# 1,1,1,1,1");
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        let result = sum_possible_arrangements(input);
        assert_eq!(result, 7792);
    }

    const INPUT0: &str =
        "#.#.### 1,1,3
        .#...#....###. 1,1,3
        .#.###.#.###### 1,3,1,6
        ####.#...#... 4,1,1
        #....######..#####. 1,6,5
        .###.##....# 3,2,1";

    // can we parse the input correctly?
    // validate the run lengths for a non-ambiguous line
    #[test]
    fn test_run_lengths() {
        let input = INPUT0;
        let (spring_map, run_lengths) = parse_input(input);
        for (map_row, run_row) in spring_map.iter().zip(run_lengths.iter()) {
            let computed_run_lengths = compute_run_lengths(map_row);
            assert_eq!(computed_run_lengths, *run_row);
        }
    }

    // can we validate the lines correctly?
    #[test]
    fn test_line_validation() {
        let input = INPUT0;
        let (spring_map, run_lengths) = parse_input(input);
        for (map_row, run_row) in spring_map.iter().zip(run_lengths.iter()) {
            assert!(is_good_line(map_row, run_row));
        }
    }

    // parse AMBIGUOUS line and compute permutations
    #[test]
    fn test_ambiguous_line() {
        let input = "???.### 1,1,3";
        let (map_row, run_row) = parse_line(input);

        let n = count_dfs_permutations(&map_row, &run_row); //, is_good_line);
        assert_eq!(n, 1);
    }

    // #[test]
    // fn test_memoized_validator() {
    //     let input = INPUT0;
    //     let (spring_map, run_lengths) = parse_input(input);

    //     let mut validator = memoize(is_good_line);
    //     for (map_row, run_row) in spring_map.iter().zip(run_lengths.iter()) {
    //         assert!(validator(map_row, run_row));
    //     }
    // }

    #[test]
    fn test_sample_input() {
        let input = INPUT1;
        let result = sum_possible_arrangements(input);
        assert_eq!(result, 21);
    }
}

