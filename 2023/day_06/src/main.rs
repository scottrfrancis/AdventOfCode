/*** Day 6
 * 
 */

use std::ops::Range;

fn get_range_race_winners( time: u32, distance: u32 ) -> Range<u32> {
    // distance travelled is a function of charging time
    // charging time is taken from the total race time
    // so `d` = `time`*charging_time - charging_time^2
    // we want to find the charging time range where 
    // `d` is greater than `distance`
    // solve for charging_time: (using quadratic formula)
    // charging_time = (time +/- sqrt(time^2 - 4*distance)) / 2
    let discriminant = time.pow(2) - 4*distance;
    if discriminant < 0 {
        panic!("No solution");
    }

    // 'cheat' the solution 'in' by epsilon so that the interval is a winner not a tie
    let charging_time_start = (time as f64 - (discriminant as f64).sqrt()) / 2.0;
    let charging_time_end = (time as f64 + (discriminant as f64).sqrt()) / 2.0;

    if charging_time_start < 0.0 || charging_time_end < 0.0 {
        panic!("No solution");
    }

    let winning_start = if charging_time_start.fract() == 0.0 { 
        charging_time_start.ceil() as u32 + 1 
    } else { 
        charging_time_start.ceil() as u32 
    };
    let winning_end = if charging_time_end.fract() == 0.0 {
        charging_time_end.floor() as u32
    } else { 
        charging_time_end.floor() as u32 + 1
    };

    Range{start: winning_start, end: winning_end}
}

fn get_winning_range_product( input: &str ) -> u32 {
    // parse input into a vector of (time, distance) tuples
    let mut lines = input.lines();
    let time_line = lines.next().unwrap().trim();
    let distance_line = lines.next().unwrap().trim();


    let times: Vec<u32> = time_line.split(":").nth(1).unwrap().trim().split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let distances: Vec<u32> = distance_line.split(":").nth(1).unwrap().trim().split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    // find the winning charging time range for each (time, distance) tuple
    let mut winning_ranges: Vec<Range<u32>> = Vec::new();
    for (time, distance) in times.iter().zip(distances.iter()) {
        let winning_range = get_range_race_winners(*time, *distance);
        winning_ranges.push(winning_range);
    }

    // calculate the product of the winning charging time ranges
    let mut product = 1;
    for range in winning_ranges {
        product *= range.end - range.start;
    }

    product
}

fn main() {
    let input  = include_str!("../input.txt");
    let product = get_winning_range_product(input);
    println!("Product of winning charging time ranges: {}", product);
}


#[cfg(test)]
mod tests {
    use super::*;


    const INPUT: &str =
            "Time:      7  15   30
            Distance:  9  40  200";

    #[test]
    fn test_get_range_race_winners() {
        let winners = get_range_race_winners(7, 9);
        assert_eq!(winners, Range{ start: 2, end: 6 });

        assert_eq!(get_range_race_winners(15, 40), Range{ start: 4, end: 12 });

        assert_eq!(get_range_race_winners(30, 200), Range{ start: 11, end: 20 });
    }

    #[test]
    fn test_winning_range_product() {
        let winning_product = get_winning_range_product(INPUT);
        assert_eq!(winning_product, 288);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        let winning_product = get_winning_range_product(input);
        assert_eq!(winning_product, 512295);
    }

}
