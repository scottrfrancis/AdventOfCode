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


fn main() {
    let input  = include_str!("../input.txt");

    let sum = sum_part_numbers(input);
    println!("\nSum of part numbers: {}", sum);
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
}
