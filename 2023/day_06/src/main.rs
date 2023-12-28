/*** Day 6
 * 
 */

use std::ops::Range;

fn get_range_race_winners( time: u128, distance: u128 ) -> Range<u128> {
    // distance travelled is a function of charging time
    // charging time is taken from the total race time
    // so `d` = `time`*charging_time - charging_time^2
    // we want to find the charging time range where 
    // `d` is greater than `distance`
    // solve for charging_time: (using quadratic formula)
    // charging_time = (time +/- sqrt(time^2 - 4*distance)) / 2
    let discriminant = (time as f64).powf(2.0) - 4.0*(distance as f64);
    if discriminant < 0.0 {
        panic!("No solution");
    }

    // 'cheat' the solution 'in' by epsilon so that the interval is a winner not a tie
    let charging_time_start = (time as f64 - (discriminant).sqrt()) / 2.0;
    let charging_time_end = (time as f64 + (discriminant).sqrt()) / 2.0;

    if charging_time_start < 0.0 || charging_time_end < 0.0 {
        panic!("No solution");
    }

    let winning_start = if charging_time_start.fract() == 0.0 { 
        charging_time_start.ceil() as u128 + 1 
    } else { 
        charging_time_start.ceil() as u128 
    };
    let winning_end = if charging_time_end.fract() == 0.0 {
        charging_time_end.floor() as u128
    } else { 
        charging_time_end.floor() as u128 + 1
    };

    Range{start: winning_start, end: winning_end}
}

fn get_winning_range_product( input: &str, ignore_spaces: bool) -> u128 {
    // parse input into a vector of (time, distance) tuples
    let mut lines = input.lines();
    let time_line = lines.next().unwrap().trim();
    let distance_line = lines.next().unwrap().trim();


    let mut times: Vec<u128> = time_line.split(":").nth(1).unwrap().trim().split_whitespace()
        .map(|s| s.parse::<u128>().unwrap())
        .collect();
    let mut distances: Vec<u128> = distance_line.split(":").nth(1).unwrap().trim().split_whitespace()
        .map(|s| s.parse::<u128>().unwrap())
        .collect();

    if ignore_spaces {
        // combine times and distances into single values
        let time_strings: Vec<String>= times.iter().map(|x| x.to_string()).collect();
        let time_str = time_strings.join(""); 
        let agg_time = time_str.parse::<u128>().unwrap();

        println!("TIME CHECK: {} ?= {}", agg_time, time_str);

        let dist_strings: Vec<String> = distances.iter().map(|x| x.to_string()).collect();
        let dist_str: String = dist_strings.join("");
        let agg_distance = dist_str.parse::<u128>().unwrap();

        println!("DIST CHECK: {} ?= {}", agg_distance, dist_str);

        times.truncate(0);
        times.push(agg_time);
        distances.truncate(0);
        distances.push(agg_distance);
    }


    // find the winning charging time range for each (time, distance) tuple
    let mut winning_ranges: Vec<Range<u128>> = Vec::new();
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
    let product = get_winning_range_product(input, false);
    println!(" PART I:  Product of winning charging time ranges: {}", product);

    let product = get_winning_range_product(input, true);
    println!(" PART II:  Product of winning charging time ranges: {}", product);

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
        let winning_product = get_winning_range_product(INPUT, false);
        assert_eq!(winning_product, 288);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        let winning_product = get_winning_range_product(input, false);
        assert_eq!(winning_product, 512295);
    }

    #[test]
    fn test_long_race() {
        let winners = get_range_race_winners(71530, 940200);
        assert_eq!(winners, Range{ start: 14, end: 71517 });
    }

    #[test]
    fn test_agg_vector() {
        let winning_product = get_winning_range_product(INPUT, true);
        assert_eq!(winning_product, 71503);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input.txt");
        let winning_product = get_winning_range_product(input, true);
        assert_eq!(winning_product, 36530883);
    }

}
