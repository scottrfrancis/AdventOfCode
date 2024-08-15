/*** Day 11
 * 
 * Rectangular Distance - run pair-wise
 *  
 */
use core::cmp::*;
use std::collections::HashSet;


 fn num_rows(input: &str) -> usize {
    input.lines().count()
 }

 fn num_cols(input: &str) -> usize {
    input.lines().next().unwrap().trim().len()
 }

fn line_has_no_galaxies(line: &str) -> bool {
    line.chars().all(|c| c == '.')
}

fn transpose(input: &str) -> String {
    let mut transposed = String::new();

    for i in 0..num_cols(input) {
        let new_row = input.lines().map(|l| l.trim().chars().nth(i).unwrap()).collect::<String>();
        transposed.push_str(format!("{}{}", new_row, '\n').as_str());
    }

    transposed
}


fn dialate_rows(input: &str) -> String {
    let mut dialated = String::new();

    for line in input.lines() {
        let line = line.trim();
        dialated.push_str(format!("{}{}", line, '\n').as_str());

        // if line is empty add a second copy of the line
        if line_has_no_galaxies(line) {
            dialated.push_str(format!("{}{}", line, '\n').as_str());
        }
    }

    dialated
}

fn dialate_map(input: &str) -> String {
    let row_dialated = dialate_rows(input);
    let transposed = transpose(&row_dialated);
    let col_dialated = dialate_rows(&transposed);

    transpose(&col_dialated)
}

fn with_galaxy(g_ch: char) -> impl Fn(&str) -> Vec<(usize, usize)> {
    let find_galaxies = move |input: &str| {
        let mut galaxies = Vec::new();

        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.trim().chars().enumerate() {
                if c == g_ch {
                    galaxies.push((row, col));
                }
            }
        }

        galaxies
    };

    find_galaxies
}

fn find_warped_rows(input: &str) -> Vec<usize> {
    let mut warped_rows = Vec::new();

    for (i, line) in input.lines().enumerate() {
        if line_has_no_galaxies(line.trim()) {
            warped_rows.push(i);
        }
    }

    warped_rows
}

fn find_warped_cols(input: &str) -> Vec<usize> {
    let mut warped_cols = Vec::new();

    let transposed = transpose(input);
    for (i, line) in transposed.lines().enumerate() {
        if line_has_no_galaxies(line.trim()) {
            warped_cols.push(i);
        }
    }

    warped_cols
}

fn rect_dist(galaxies: &Vec<(usize, usize)>, warped_rows: &Vec<usize>, warped_cols: &Vec<usize>, weight: usize) -> Vec<((usize, usize), usize)> {
    let mut shortest_paths: Vec<((usize, usize), usize)> = Vec::new();

    let rows_to_warp = warped_rows.iter().map(|&x| x as i32).collect::<HashSet<i32>>();
    let cols_to_warp = warped_cols.iter().map(|&x| x as i32).collect::<HashSet<i32>>();

    for (i, galaxy) in galaxies.iter().enumerate() {
        for (j, other_galaxy) in galaxies.iter().enumerate() {
            if j <= i {
                continue;
            }

            let top = min(galaxy.0, other_galaxy.0);
            let bottom = max(galaxy.0, other_galaxy.0);
            let left = min(galaxy.1, other_galaxy.1);
            let right = max(galaxy.1, other_galaxy.1);
            let rows: HashSet<usize> = (top..=bottom).collect();
            let cols: HashSet<usize> = (left..=right).collect();

            let warp_rows_count = rows.intersection(&warped_rows.iter().cloned().collect()).count();
            let warp_cols_count = cols.intersection(&warped_cols.iter().cloned().collect()).count();

            let distance = (rows.len() - 1 - warp_rows_count + cols.len() - 1 - warp_cols_count)*1 +
                                 (warp_rows_count + warp_cols_count)*weight;

            shortest_paths.push(((i, j), distance as usize));
        }
    }

    shortest_paths
}

fn main() {
    println!("Day 11");

    println!("Part 1");
    let input = include_str!("../input.txt");
    
    let shortest_paths = rect_dist(
        &(with_galaxy('#')(&input)), 
        &(find_warped_rows(input)), &(find_warped_cols(input)), 
        2);

    let sum_paths = shortest_paths.iter().map(|&x| x.1 as u64).sum::<u64>();
    println!("Sum of shortest paths: {}", sum_paths);

    println!("Part 2");
    
    let shortest_paths = rect_dist(
        &(with_galaxy('#')(&input)), 
        &(find_warped_rows(input)), &(find_warped_cols(input)), 
        1_000_000);

    let sum_paths = shortest_paths.iter().map(|&x| x.1 as u64).sum::<u64>();
    println!("Sum of shortest paths: {}", sum_paths);    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_warp_factor() {
        let input = INPUT1;
        let shortest_paths = rect_dist(
            &(with_galaxy('#')(&input)), 
            &(find_warped_rows(input)), &(find_warped_cols(input)), 
            10);

        let sum_paths = shortest_paths.iter().map(|&x| x.1 as u64).sum::<u64>();
        assert_eq!(sum_paths, 1030);       

        let shortest_paths = rect_dist(
            &(with_galaxy('#')(&input)), 
            &(find_warped_rows(input)), &(find_warped_cols(input)), 
            100);

        let sum_paths = shortest_paths.iter().map(|&x| x.1 as u64).sum::<u64>();
        assert_eq!(sum_paths, 8410);                
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        let shortest_paths = rect_dist(
            &(with_galaxy('#')(&input)), 
            &(find_warped_rows(input)), &(find_warped_cols(input)), 
            2);

        let sum_paths = shortest_paths.iter().map(|&x| x.1 as u64).sum::<u64>();
        assert_eq!(sum_paths, 9799681);
    }

    const INPUT1: &str = 
       "...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....";

    #[test]
    fn test_sample_input() {
        let warped_rows = find_warped_rows(INPUT1);
        let warped_cols = find_warped_rows(transpose(INPUT1).as_str());

        let galaxies = with_galaxy('#')(&INPUT1);
        let shortest_paths = rect_dist(&galaxies, &warped_rows, &warped_cols, 2);

        assert_eq!(shortest_paths.len(), 36);
        let paths = shortest_paths.iter().map(|&x| x.1 as u64).collect::<Vec<u64>>();
        assert_eq!(shortest_paths.iter().map(|&x| x.1 as u64).sum::<u64>(), 374);
    }

    #[test]
    fn test_find_warped_rows() {
        let warped_rows = find_warped_rows(INPUT1);
        assert_eq!(warped_rows.len(), 2);

        let transposed = transpose(INPUT1);
        let warped_cols = find_warped_rows(&transposed);
        assert_eq!(warped_cols.len(), 3);
    }

    #[test]
    fn test_find_galaxies() {
        // let dialated = dialate_map(INPUT1);

        let finder = with_galaxy('#');
        let galaxies = finder(&INPUT1);
        assert_eq!(galaxies.len(), 9);
    }   

    // #[test]
    // fn test_dialate_map() {
    //     let dialated = dialate_map(INPUT1);
    //     assert_eq!(num_rows(&dialated), 12);
    //     assert_eq!(num_cols(&dialated), 13);
    // }

    // #[test]
    // fn test_dialate_rows() {
    //     let dialated_rows = dialate_rows(INPUT1);
    //     assert_eq!(num_rows(&dialated_rows), 12);
    // }

    #[test]
    fn test_transpose() {
        let transposed = transpose(INPUT1);
        assert_eq!(num_cols(&transposed), 10);
    }

        
}

