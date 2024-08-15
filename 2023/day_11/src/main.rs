/*** Day 11
 * 
 * Rectangular Distance - run pair-wise
 *  
 */

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

fn rect_dist(galaxies: &Vec<(usize, usize)>) -> Vec<((usize, usize), usize)> {
    let mut shortest_paths: Vec<((usize, usize), usize)> = Vec::new();

    for (i, galaxy) in galaxies.iter().enumerate() {
        for (j, other_galaxy) in galaxies.iter().enumerate() {
            if j <= i {
                continue;
            }

            let distance = (galaxy.0 as i32 - other_galaxy.0 as i32).abs() + (galaxy.1 as i32 - other_galaxy.1 as i32).abs(); 

            shortest_paths.push(((i, j), distance as usize));
        }
    }

    shortest_paths
}

fn main() {
    println!("Day 11");
    let input = include_str!("../input.txt");
    let dialated = dialate_map(input);
 
}


#[cfg(test)]
mod tests {
    use super::*;

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
        let galaxies = with_galaxy('#')(dialate_map(INPUT1).as_str());
        let shortest_paths = rect_dist(&galaxies);

        assert_eq!(shortest_paths.len(), 36);
        let paths = shortest_paths.iter().map(|&x| x.1 as u64).collect::<Vec<u64>>();
        assert_eq!(shortest_paths.iter().map(|&x| x.1 as u64).sum::<u64>(), 374);
    }

    #[test]
    fn test_find_galaxies() {
        let dialated = dialate_map(INPUT1);

        let finder = with_galaxy('#');
        let galaxies = finder(&dialated);
        assert_eq!(galaxies.len(), 9);
    }   

    #[test]
    fn test_dialate_map() {
        let dialated = dialate_map(INPUT1);
        assert_eq!(num_rows(&dialated), 12);
        assert_eq!(num_cols(&dialated), 13);
    }

    #[test]
    fn test_dialate_rows() {
        let dialated_rows = dialate_rows(INPUT1);
        assert_eq!(num_rows(&dialated_rows), 12);
    }

    #[test]
    fn test_transpose() {
        let transposed = transpose(INPUT1);
        assert_eq!(num_cols(&transposed), 10);
    }

        
}

