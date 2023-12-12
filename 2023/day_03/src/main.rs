use std::collections::HashSet;


fn is_symbol(c: char) -> bool {
    let is_it = !c.is_alphanumeric() && c != '.';
    assert_eq!(is_it, !c.is_digit(10) && c != '.');
    is_it
}

fn find_used_parts(buffer: &Vec<&str>, parts: &mut HashSet<u32>) -> u32 {
    let mut sum: u32 = 0;
    // find the bounds of the number
    let mut i = 0;
    let above = buffer[0];
    let line = buffer[1];
    let below = buffer[2];
    let line_length = line.len();
    while i < line_length {
        if !line.chars().nth(i).unwrap().is_digit(10) {
            i += 1;
            continue;
        }
    
        let left_bound: usize = i;
        let mut right_bound: usize = left_bound;

        // find the right bound
        while right_bound < line_length && line.chars().nth(right_bound).unwrap().is_digit(10) {
            right_bound += 1;
        }
        // right_bound -= 1;
        let numstr = &line[left_bound..right_bound];
        println!("Found {}", numstr);
        let num = numstr.parse::<u32>().unwrap();

        println!("Found {}", num);

        // now is the number used?  
        // check left an right of the bounds
        let left = if left_bound <= 0 { 0 } else { left_bound - 1 };
        let right = if right_bound >= line_length - 1 { line_length - 1 } else { right_bound };
        println!("Checking bounds: \n{}\n{}\n{}", 
            above[left..=right].to_string(), line[left..=right].to_string(), below[left..=right].to_string());
        for j in left..=right {
            if is_symbol(above.chars().nth(j).unwrap()) || 
                is_symbol(line.chars().nth(j).unwrap()) || 
                is_symbol(below.chars().nth(j).unwrap()) {
                println!("USED!\n");
                parts.insert(num);
                // count duplicate parT numbers!  Doesn't make sense.. .but that how it worked!
                sum += num;
                break;
            }
        }
        
        i = right_bound + 1;
    }

    sum
}

fn find_number_around(line:&str, idx: usize) -> (u32, usize, usize) {
    let mut left_bound: usize = idx;
    while left_bound > 0 && line.chars().nth(left_bound - 1).unwrap().is_digit(10) {
        left_bound -= 1;
    }
    // find right bound of number
    let mut right_bound: usize = idx;
    while right_bound < line.len() - 1 && line.chars().nth(right_bound + 1).unwrap().is_digit(10) {
        right_bound += 1;
    }

    let numstr = &line[left_bound..=right_bound];
    let num = numstr.parse::<u32>().unwrap();
    
    (num, left_bound, right_bound)
}

fn grind_gears(buffer: &Vec<&str>) -> u32 {
    let mut sum: u32 = 0;
    // find the bounds of the number
    let mut i = 0;
    let above = buffer[0];
    let line = buffer[1];
    let below = buffer[2];
    let line_length = line.len();

    while i < line_length {
        let c = line.chars().nth(i).unwrap();
        if !is_symbol(c) {
            i += 1;
            continue;
        }

        // look for digits of part numbers around symbol
        let left_neighbor: usize = if i > 0  { i - 1 } else { 0 };
        let right_neighbor: usize = if i < line_length - 1 { i + 1 } else { line_length - 1 };
        let mut j = right_neighbor ;
        // look above
        while j >= left_neighbor {
            if above.chars().nth(j).unwrap().is_digit(10) {
                let (num, start, _end) = find_number_around(above, j);
                print!("{} ", num);
                j = start;
                sum += num;
            }
            j = if j > 0 { j - 1 } else { 0 };
        }

        // look below
        j = right_neighbor;
        while j >= left_neighbor {
            if below.chars().nth(j).unwrap().is_digit(10) {
                let (num, start, _end) = find_number_around(below, j);
                print!("{} ", num);
                j = start;
                sum += num;
            }
            j = if j > 0 { j - 1 } else { 0 };
        }

        // left
        if line.chars().nth(left_neighbor).unwrap().is_digit(10) {
            let (num, _start, _end) = find_number_around(line, left_neighbor);
            print!("{} ", num);
            sum += num;
        }

        // right
        if line.chars().nth(right_neighbor).unwrap().is_digit(10) {
            let (num, _start, _end) = find_number_around(line, right_neighbor);
            print!("{} ", num);
            sum += num;
        }
        println!();
        i += 1;
    }

    sum
}


fn grind_gear_pairs(buffer: &Vec<&str>) -> u32 {
    let mut sum: u32 = 0;
    // find the bounds of the number
    let mut i = 0;
    let above = buffer[0];
    let line = buffer[1];
    let below = buffer[2];
    let line_length = line.len();

    while i < line_length {
        let c = line.chars().nth(i).unwrap();
        if !is_symbol(c) && c != '*' {
            i += 1;
            continue;
        }
        let mut nearby_numbers: Vec<u32> = Vec::new();

        // look for digits of part numbers around symbol
        let left_neighbor: usize = if i > 0  { i - 1 } else { 0 };
        let right_neighbor: usize = if i < line_length - 1 { i + 1 } else { line_length - 1 };
        let mut j = right_neighbor ;
        // look above
        while j >= left_neighbor {
            if above.chars().nth(j).unwrap().is_digit(10) {
                let (num, start, _end) = find_number_around(above, j);
                print!("{} ", num);
                j = start;
                nearby_numbers.push(num);
            }
            j = if j > 0 { j - 1 } else { 0 };
        }

        // look below
        j = right_neighbor;
        while j >= left_neighbor {
            if below.chars().nth(j).unwrap().is_digit(10) {
                let (num, start, _end) = find_number_around(below, j);
                print!("{} ", num);
                j = start;
                nearby_numbers.push(num);
            }
            j = if j > 0 { j - 1 } else { 0 };
        }

        // left
        if line.chars().nth(left_neighbor).unwrap().is_digit(10) {
            let (num, _start, _end) = find_number_around(line, left_neighbor);
            print!("{} ", num);
            nearby_numbers.push(num);
        }

        // right
        if line.chars().nth(right_neighbor).unwrap().is_digit(10) {
            let (num, _start, _end) = find_number_around(line, right_neighbor);
            print!("{} ", num);
            nearby_numbers.push(num);
        }

        if nearby_numbers.len() == 2 {
            sum += nearby_numbers[0] * nearby_numbers[1];
        }

        println!();
        i += 1;
    }

    sum
}

fn sum_part_numbers(input: &str) -> u32 {
    // keep a buffer of 3 lines
    // padded above, below with '.'
    let mut buffer: Vec<&str> = Vec::new();
    let line_length = input.lines().next().unwrap().len();
    let pad_line: String = ".".repeat(line_length);
    buffer.push(&pad_line);

    let mut part_numbers: HashSet<u32> = HashSet::new();
    let mut all_sum = 0;

    for line in input.lines() {
        // let line_chars: Vec<char>  = line.chars().collect();
        buffer.push(line.trim());

        if buffer.len() < 3 {
            continue;
        } else if buffer.len() > 3 {
            buffer.remove(0);
        }

        // process the middle line of the buffer
        println!("Processing line: {}", buffer[1]);
        all_sum += find_used_parts(&buffer, &mut part_numbers);
    }
    buffer.push(&pad_line);
    if buffer.len() > 3 {
        buffer.remove(0);
    }
    all_sum += find_used_parts(&buffer, &mut part_numbers);

    all_sum
}

fn sum_gear_products(input: &str) -> u32 {
    // keep a buffer of 3 lines
    // padded above, below with '.'
    let mut buffer: Vec<&str> = Vec::new();
    let line_length = input.lines().next().unwrap().len();
    let pad_line: String = ".".repeat(line_length);
    buffer.push(&pad_line);

    let mut sum = 0;

    for line in input.lines() {
        // let line_chars: Vec<char>  = line.chars().collect();
        buffer.push(line.trim());

        if buffer.len() < 3 {
            continue;
        } else if buffer.len() > 3 {
            buffer.remove(0);
        }

        // process the middle line of the buffer
        println!("Processing line: {}", buffer[1]);
        sum += grind_gear_pairs(&buffer);
    }
    buffer.push(&pad_line);
    if buffer.len() > 3 {
        buffer.remove(0);
    }
    sum += grind_gear_pairs(&buffer);
    
    sum
}

fn sum_gear_ratios(input: &str) -> u32 {
    // keep a buffer of 3 lines
    // padded above, below with '.'
    let mut buffer: Vec<&str> = Vec::new();
    let line_length = input.lines().next().unwrap().len();
    let pad_line: String = ".".repeat(line_length);
    buffer.push(&pad_line);

    let mut sum = 0;

    for line in input.lines() {
        // let line_chars: Vec<char>  = line.chars().collect();
        buffer.push(line.trim());

        if buffer.len() < 3 {
            continue;
        } else if buffer.len() > 3 {
            buffer.remove(0);
        }

        // process the middle line of the buffer
        println!("Processing line: {}", buffer[1]);
        sum += grind_gears(&buffer);
    }
    buffer.push(&pad_line);
    if buffer.len() > 3 {
        buffer.remove(0);
    }
    sum += grind_gears(&buffer);
    
    sum
}


fn main() {
    let input  = include_str!("../input.txt");

    let sum = sum_gear_products(input);
    println!("\nSum of gear products: {}", sum);   
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_part_numbers() {
        let input =
            "467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..";

        assert_eq!(sum_part_numbers(input), 4361);
    }

    #[test]
    fn test_part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(sum_part_numbers(input), 527364);
    }

    #[test]
    fn test_gear_ratios() {
        let input =
            "467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..";

        assert_eq!(sum_gear_ratios(input), 4361);
    }

    #[test]
    fn test_grind_part1() {
        let input = include_str!("../input.txt");
        assert_eq!(sum_gear_ratios(input), 527364);
    }

    #[test]
    fn test_gear_ratio_products() {
        let input =
            "467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..";

        assert_eq!(sum_gear_products(input), 467835);
    }

    #[test]
    fn test_grind_part2() {
        let input = include_str!("../input.txt");
        assert_eq!(sum_gear_products(input), 527364);
    }
}
