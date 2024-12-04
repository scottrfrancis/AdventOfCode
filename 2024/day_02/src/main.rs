use polars::prelude::*; // if the crate polars-core is used directly
use regex::Regex;

fn parse_line(line: &str) -> Vec<i32> {
    let re = Regex::new(r"\s+").unwrap();
    let row = re.split(line.trim())
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    row
}

fn is_level_safe(level: Vec<i32>, threshold: i32) -> bool {
    let dr = level.windows(2)
        .map(|x| (x[0] - x[1]))
        .collect::<Vec<i32>>();
    let length = dr.len();

    // are all values in dr positive or zero?
    let increasing_count = dr.iter()
        .map(|x| x > &0)
        .collect::<Vec<bool>>()
        .into_iter()
        .filter(|x| *x)
        .count();

    let decreasing_count = dr.iter()
        .map(|x| x < &0)
        .collect::<Vec<bool>>()
        .into_iter()
        .filter(|x| *x)
        .count();

    let zero_count = dr.iter()
        .map(|x| x == &0)
        .collect::<Vec<bool>>()
        .into_iter()
        .filter(|x| *x)
        .count();

    let min_valid = length;
    if (zero_count > 0) ||
        ((increasing_count < min_valid) &&
        (decreasing_count < min_valid)) {
        return false;
    }

    let result: i32 = dr.iter()
        .map(|x| if x.abs() > threshold { 1 } else { 0 })
        .collect::<Vec<i32>>()
        .into_iter()
        .sum();
    
    result == 0
}

fn main() {
    // read input file
    let input = include_str!("../input.txt");

    // part 1   
    let mut safe_levels = 0;
    for line in input.lines() {
        let row = parse_line(line);
        if is_level_safe(row, 3) {
            safe_levels += 1;
        }
    }
    println!("Safe levels: {}", safe_levels);

    // part 2
    let mut safe_levels = 0;
    for line in input.lines() {
        let row = parse_line(line);
        for i in 0..row.len() {
            let mut slice = row.clone();
            slice.remove(i);
            if is_level_safe(slice.to_vec(), 3) {
                safe_levels += 1;
                break;
            }
        }
    }
    println!("Safe levels: {}", safe_levels);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_choose_n_safe() {
        let input = INPUT;
        let mut safe_levels = 0;
        for line in input.lines() {
            let row = parse_line(line);
            for i in 0..row.len() {
                let mut slice = row.clone();
                slice.remove(i);
                if is_level_safe(slice.to_vec(), 3) {
                    safe_levels += 1;
                    break;
                }
            }
        }
        assert_eq!(safe_levels, 4);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        let mut safe_levels = 0;
        for line in input.lines() {
            let row = parse_line(line);
            if is_level_safe(row, 3) {
                safe_levels += 1;
            }
        }
        assert_eq!(safe_levels, 402);
    }

    #[test]
    fn test_sample() {
        let mut safe_levels = 0;

        for line in INPUT.lines() {
            let row = parse_line(line);
            if is_level_safe(row, 3) {
                safe_levels += 1;
            }
        }

        assert_eq!(safe_levels, 2);
    }

    #[test]
    fn test_threshold() {
        // let dr = vec![1, 2, 2, 1];
        let threshold = 1;
        let result = is_level_safe(vec![7, 6, 4, 2, 1], threshold);
        assert_eq!(result, false);

        assert!(!is_level_safe(vec![1, 2, 7, 8, 9], 3));
        assert!(!is_level_safe(vec![9, 7, 6, 2, 1], 3));
        assert!(!is_level_safe(vec![1, 3, 2, 4, 5], 3));
        assert!(!is_level_safe(vec![8, 6, 4, 4, 1], 3));
        assert!( is_level_safe(vec![1, 3, 6, 7, 9], 3));
    }

    #[test]
    fn test_derivative() {
        let input = "7 6 4 2 1";
        let row = parse_line(input);
        assert_eq!(row, vec![7, 6, 4, 2, 1]);   

        let d_row = row.windows(2)
            .map(|x| (x[0] - x[1]))
            .collect::<Vec<i32>>();
        assert_eq!(d_row, vec![1, 2, 2, 1]);
    }   

    const INPUT: &str = 
       "7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9";

    #[test]
    fn test_parse_lines() {
        let mut lines = INPUT.lines();
        let row = parse_line(lines.next().unwrap());
        assert_eq!(row, vec![7, 6, 4, 2, 1]);

        let row = parse_line(lines.next().unwrap());
        assert_eq!(row, vec![1, 2, 7, 8, 9]);

        let row = parse_line(lines.next().unwrap());
        assert_eq!(row, vec![9, 7, 6, 2, 1]);

        assert_eq!(parse_line(lines.next().unwrap()), vec![1, 3, 2, 4, 5]);
        assert_eq!(parse_line(lines.next().unwrap()), vec![8, 6, 4, 4, 1]);
        assert_eq!(parse_line(lines.next().unwrap()), vec![1, 3, 6, 7, 9]);
    }   

}