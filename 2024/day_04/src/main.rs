use std::fmt;


#[derive(Debug)]
struct Grid<T> {
    width: usize,
    height: usize,
    cells: Vec<Vec<T>>,
}

impl<T: Clone> Grid<T> {
    fn new(width: usize, height: usize, default_value: T) -> Self {
        let cells = vec![vec![default_value; width]; height];
        Grid { width, height, cells }
    }

    fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.width && y < self.height {
            Some(&self.cells[y][x])
        } else {
            None
        }
    }

    fn set(&mut self, x: usize, y: usize, value: T) {
        if x < self.width && y < self.height {
            self.cells[y][x] = value;
        }
    }

    fn add_row(&mut self, default_value: T) {
        let new_row = vec![default_value; self.width];
        self.cells.push(new_row);
        self.height += 1;
    }

    fn append_row(&mut self, row: Vec<T>) {
        if row.len() == self.width {
            self.cells.push(row);
            self.height += 1;
        }
    }
}

impl<T: fmt::Display> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.cells {
            for cell in row {
                write!(f, "{} ", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn build_grid(input: &str) -> Grid<char> {
    // get length of first line
    let width = input.lines().next().unwrap().len();
    let mut grid = Grid::new(width, 0, '.');

    for line in input.lines() {
        let row: Vec<char> = line.trim().chars().collect();
        grid.append_row(row);
    }
    grid
}

// get all the n-char strings around a point in the grid
fn get_strings_around_point(grid: &Grid<char>, x: usize, y: usize, n: usize) -> Vec<String> {
    let mut strings = Vec::new();
    let width = grid.width;
    let height = grid.height;

    // horizontal
    if x + n <= width {
        strings.push(grid.cells[y][x..x+n].iter().collect());
    }
    if x >= n - 1 {
        let s: String = grid.cells[y][x-(n-1)..x+1].iter().collect();
        strings.push(s.chars().rev().collect());
    }

    // vertical
    if y + n <= height {
        strings.push(grid.cells[y..y+n].iter().map(|row| row[x]).collect());
    }
    if y >= n - 1 {
        let s: String = grid.cells[y-(n-1)..y+1].iter().map(|row| row[x]).collect();
        strings.push(s.chars().rev().collect());
    }

    // diagonal
    if x + n <= width && y + n <= height {
        // SE -- in order
        strings.push((0..n).map(|i| grid.cells[y+i][x+i]).collect());
    }
    if x >= n - 1 && y >= n - 1 {
        // NW 
        let s: String = (0..n).map(|i| grid.cells[y-i][x-i]).collect();
        // strings.push(s.chars().rev().collect());
        strings.push(s);
    }
    if x + n <= width && y >= n - 1 {
        // NE 
        let s: String = (0..n).map(|i| grid.cells[y-i][x+i]).collect();
        // strings.push(s.chars().rev().collect());
        strings.push(s);
    }
    if x >= n - 1 && y + n <= height {
        // SW
        let s: String = (0..n).map(|i| grid.cells[y+i][x-i]).collect();
        // strings.push(s.chars().rev().collect());
        strings.push(s);
    }

    strings
}

fn count_in_grid(grid: &Grid<char>, target: &str) -> usize {
    let mut count = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            let ch = grid.cells[y][x];
            if ch != target.chars().next().unwrap() {
                continue;
            }
            let strings = get_strings_around_point(&grid, x, y, target.len());
            for s in strings {
                if s == target {
                    count += 1;
                }
            }
        }
    }
    count
}

fn get_x_strings_centered_on_point(grid: &Grid<char>, x: usize, y: usize, n: usize) -> Vec<String> {
    let mut strings = Vec::new();
    let width = grid.width;
    let height = grid.height;

    if x < n/2 || y < n/2 || x >= width - n/2 || y >= height - n/2 {
        return strings;
    }

    // NW - SE
    let start: i32 = -(n as i32)/2;
    let stop: i32 = (n as i32)/2;
    let s: String = (start..=stop).map(|i| grid.cells[(y as i32 + i) as usize][(x as i32 + i) as usize]).collect();
    strings.push(s);
    // strings.push(s.chars().rev().collect());

    // SW - NE
    let s: String = (start..=stop).map(|i| grid.cells[(y as i32 - i) as usize][(x as i32 + i) as usize]).collect();
    strings.push(s);
    // strings.push(s.chars().rev().collect());

    strings
}

fn count_x_mas(grid: &Grid<char>) -> usize {
    let mut count = 0;
    for y in 1..grid.height - 1 {
        for x in 1..grid.width - 1 {
            let ch = grid.cells[y][x];
            if ch != 'A' {
                continue;
            }
            let strings = get_x_strings_centered_on_point(&grid, x, y, 3);
            assert!(strings.len() == 2);
            
            if (strings[0] == "MAS" || strings[0] == "SAM") &&
               (strings[1] == "SAM" || strings[1] == "MAS") {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let input = include_str!("../input.txt");
    let grid = build_grid(input);
    // println!("{}", grid);

    let mut count = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            let ch = grid.cells[y][x];
            // println!("({}, {}) - {}", x, y, ch);
            if ch != 'X' {
                continue;
            }
            let strings = get_strings_around_point(&grid, x, y, 4);
            for s in strings {
                if s == "XMAS" {
                    // println!("Found XMAS at ({}, {})", x, y);
                    count += 1;
                }
            }
        }
    }

    println!("Found {} instances of XMAS", count);

    let count = count_x_mas(&grid);
    println!("Found {} instances of X-MAS", count);
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = include_str!("../input.txt");
        let grid = build_grid(input);
        let count = count_x_mas(&grid);
        assert_eq!(count, 1824);
    }

    #[test]
    fn test_count_x_mas() {
        let grid = build_grid(SAMPLE);
        println!("{}", grid);

        let count = count_x_mas(&grid);
        assert_eq!(count, 9);
    }

    #[test]
    fn test_center_strings() {
        let grid = build_grid(SAMPLE);
        println!("{}", grid);

        let strings = get_x_strings_centered_on_point(&grid, 3, 3, 3);
        assert_eq!(strings.len(), 2);

        let strings = get_x_strings_centered_on_point(&grid, 2, 1, 3);
        assert_eq!(strings.len(), 2);
        assert!(strings[0] == "MAS" || strings[0] == "SAM");
        assert!(strings[1] == "MAS" || strings[1] == "SAM");
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        let grid = build_grid(input);
        let count = count_in_grid(&grid, "XMAS");
        assert_eq!(count, 2397);
    }

    #[test]
    fn test_count_sample() {
        let grid = build_grid(SAMPLE);
        println!("{}", grid);

        let mut count = 0;
        for y in 0..grid.height {
            for x in 0..grid.width {
                let ch = grid.cells[y][x];
                // println!("({}, {}) - {}", x, y, ch);
                if ch != 'X' {
                    continue;
                }
                let strings = get_strings_around_point(&grid, x, y, 4);
                for s in strings {
                    if s == "XMAS" {
                        println!("Found XMAS at ({}, {})", x, y);
                        count += 1;
                    }
                }
            }
        }

        assert_eq!(count, 18);
    }

    #[test]
    fn test_search() {
        let grid = build_grid(SAMPLE);
        println!("{}", grid);

        let strings = get_strings_around_point(&grid, 3, 9, 4);
        assert_eq!(strings.len(), 5);

        let strings = get_strings_around_point(&grid, 0, 0, 4);
        assert_eq!(strings.len(), 3);

        let strings = get_strings_around_point(&grid, 5, 5, 4);
        assert_eq!(strings.len(), 8);

        let strings = get_strings_around_point(&grid, 9, 9, 4);
        assert_eq!(strings.len(), 3);

        let strings = get_strings_around_point(&grid, 3, 0, 4);
        assert_eq!(strings.len(), 5);

        let strings = get_strings_around_point(&grid, 6, 0, 4);
        assert_eq!(strings.len(), 5);

        let strings = get_strings_around_point(&grid, 0, 3, 4);
        assert_eq!(strings.len(), 5);

        let strings = get_strings_around_point(&grid, 0, 6, 4);
        assert_eq!(strings.len(), 5);

        let strings = get_strings_around_point(&grid, 9, 3, 4);
        assert_eq!(strings.len(), 5);

        let strings = get_strings_around_point(&grid, 9, 6, 4);
        assert_eq!(strings.len(), 5);

        let strings = get_strings_around_point(&grid, 3, 9, 4);
        assert_eq!(strings.len(), 5);

        let strings = get_strings_around_point(&grid, 6, 9, 4);
        assert_eq!(strings.len(), 5);
    }


    const SAMPLE: &str =
       "MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX";

    #[test]
    fn test_build_grid() {
        let grid = build_grid(SAMPLE);
        println!("{}", grid);

        assert_eq!(grid.width, 10);
        assert_eq!(grid.height, 10);

        assert_eq!(grid.get(0, 0), Some(&'M'));
        assert_eq!(grid.get(1, 1), Some(&'S'));
        assert_eq!(grid.get(2, 2), Some(&'X'));
        assert_eq!(grid.get(3, 3), Some(&'M'));
    }

    #[test]
    fn test_grid() {
        let mut grid = Grid::new(3, 3, 0);
        assert_eq!(grid.get(0, 0), Some(&0));
        assert_eq!(grid.get(1, 1), Some(&0));
        assert_eq!(grid.get(2, 2), Some(&0));
        assert_eq!(grid.get(3, 3), None);

        grid.set(0, 0, 1);
        grid.set(1, 1, 2);
        grid.set(2, 2, 3);
        assert_eq!(grid.get(0, 0), Some(&1));
        assert_eq!(grid.get(1, 1), Some(&2));
        assert_eq!(grid.get(2, 2), Some(&3));

        grid.add_row(4);
        assert_eq!(grid.get(0, 3), Some(&4));
        assert_eq!(grid.get(1, 3), Some(&4));
        assert_eq!(grid.get(2, 3), Some(&4));
    }
}