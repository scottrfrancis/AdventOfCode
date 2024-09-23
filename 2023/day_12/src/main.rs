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

fn replace_chars(map_row: &[char], mask: &Vec<bool>, replacements: Vec<char>) -> Vec<char> {
    let mut new_row = map_row.to_vec();
    let mut replacement_chars = replacements.clone();

    for i in 0..map_row.len() {
        if mask[i] {
            new_row[i] = replacement_chars.pop().unwrap();
        }
    }

    new_row
}

fn replacements_for_idx(n: usize, len: usize) -> Vec<char> {
    // create vector of length len, with n as binary 1s and the rest as binary 0s
    let mut replacements: Vec<char> = vec!['.'; len];
    // consider n as a binary mask on the index of replacements, 
    // set the bits that are set in n to '#'
    for i in 0..len {
        if n & (1 << i) != 0 {
            replacements[i] = '#';
        }
    }

    replacements
}


fn count_successful_permutations<F>(map_row: &[char], run_row: &[usize], mut validator: F) -> usize 
where
    F: FnMut(&[char], &[usize]) -> bool,
{
    let mut count = 0;
   
    let mask: Vec<bool> = map_row.iter().map(|&ch| ch == '?').collect();
    let num_to_try = mask.iter().filter(|&&b| b).count();
        
    if num_to_try == 0 {
        return if validator(map_row, run_row) { 1 } else { 0 };
    }
    
    let mut try_count = usize::pow(2, num_to_try as u32);
    while try_count > 0 {
        let replacements = replacements_for_idx(try_count - 1, num_to_try);
        let test_row = replace_chars(map_row, &mask, replacements);
        // print!("{:?} ", test_row); 
        if validator(&test_row, run_row) {
            count += 1;
        }

        try_count -= 1;
    }

    count
}

fn sum_possible_arrangements(input: &str) -> usize {
    let (spring_map, run_lengths) = parse_input(input);
    let mut total = 0;
    // let mut validator = memoize(is_good_line);
    let mut validator = is_good_line;

    for (map_row, run_row) in spring_map.iter().zip(run_lengths.iter()) {
        total += count_successful_permutations(map_row, run_row, &mut validator);
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
    
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = 
        "???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1";

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

        let n = count_successful_permutations(&map_row, &run_row, is_good_line);
        assert_eq!(n, 1);
    }

    #[test]
    fn test_memoized_validator() {
        let input = INPUT0;
        let (spring_map, run_lengths) = parse_input(input);

        let mut validator = memoize(is_good_line);
        for (map_row, run_row) in spring_map.iter().zip(run_lengths.iter()) {
            assert!(validator(map_row, run_row));
        }
    }

    #[test]
    fn test_sample_input() {
        let input = INPUT1;
        let result = sum_possible_arrangements(input);
        assert_eq!(result, 21);
    }
}

