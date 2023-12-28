/*** Day 5
 * 
 */

use phf::map;
use polars::prelude::*; // if the crate polars-core is used directly
use std::{collections::{HashMap}, vec};


#[derive(Debug)]
struct Map<T> {
    source: Vec<T>,
    destination: Vec<T>,
    range: Vec<T>,
}

fn read_maps<'a>(input: &'a str, maps: &mut HashMap<&'a str, Map<u32>>) {
    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        let line = line.trim();
        if line.is_empty() {
            // blank line, skip
            continue;
        }
        let map_name = line.split(": ").nth(0).unwrap().split(" ").nth(0).unwrap();
        let mut sources: Vec<u32> = vec![0];
        let mut destinations: Vec<u32> =vec![0];
        let mut ranges: Vec<u32> = vec![u32::MAX];
        // the next lines are the map itself until a blank line
        while let Some(map_line) = lines.next() {
            let map_line = map_line.trim();
            if map_line.is_empty() {
                // blank line, end of map
                break;
            }
            // parse the map line
            let map_row: Vec<_> = map_line
                .split(" ")
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            assert!(map_row.len() == 3);

            sources.push(map_row[1]);
            destinations.push(map_row[0]);
            ranges.push(map_row[2]);
        }

        maps.insert(map_name, Map{
            source: sources,
            destination: destinations,
            range: ranges,
        });
    }
    // println!("{:?}\n", maps);
}

const map_sequence: [&str; 7]  = [
    "seed-to-soil", 
    "soil-to-fertilizer", 
    "fertilizer-to-water", 
    "water-to-light", 
    "light-to-temperature", 
    "temperature-to-humidity", 
    "humidity-to-location"
];

fn lowest_location(input: &str) -> u32 {
    let mut lowest_location = u32::MAX;

    let mut maps: HashMap<&str, Map<u32>> = HashMap::new();
    read_maps(input, &mut maps);

    // this function is for part 1, so extract the seed list
    let mut seeds: Vec<u32> = Vec::new();
    let seed_map = maps.remove("seeds").unwrap();
    assert!(seed_map.source.len() > 0);
    for i in 1..seed_map.source.len() {
        seeds.push(seed_map.source[i]);
        seeds.push(seed_map.range[i]);
    }
    println!("seeds: {:?}", seeds);

    // calculate location

    for seed in seeds {
        let mut dst = seed;

        for map_name in map_sequence.iter() {
            println!("{} at {}", map_name, dst);
            let map = maps.get(map_name).unwrap();
            let val = interpolate_map(&map.source, &map.destination, &map.range, dst);
            if let Some(val) = val {
                dst = val;
            } else {
                println!("No value found for {} at {}", map_name, dst);
                break;
            }
        }
        if dst < lowest_location {
            lowest_location = dst;
        }
    }

    lowest_location
}

fn lowest_with_seed(input: &str) -> u32 {
    let mut lowest_location: u32 = u32::MAX;

    let mut maps: HashMap<&str, Map<u32>> = HashMap::new();
    read_maps(input, &mut maps);

    // walk UP the location list and run the maps backwards
    // if can walk back to a seed, if it's in the seed range map, that is the lowest

    // extract seed range from seed map
    let seed_map = maps.remove("seeds").unwrap();
    assert!(seed_map.source.len() > 0);

    let df = DataFrame::new(vec![
        Series::new("dst", seed_map.destination), 
        Series::new("range", seed_map.range)]).unwrap();
    let sdf = df.sort(["dst", "range"], false, true).unwrap();
    let seed_starts = sdf.column("dst").unwrap();
    let ranges = sdf.column("range").unwrap();
    let mut seed_ranges: Vec<(u32, u32)> = Vec::new();
    assert_eq!(seed_starts.len(), ranges.len());
    for i in 1..seed_starts.len() {
        seed_ranges.push(
            (seed_starts.u32().expect("not u32").get(i).expect("was Null"),
            ranges.u32().expect("not u32").get(i).expect("was Null"))
        );            
    }
    // seed_ranges now has tuples of start and length

    // get location ranges
    let map = maps.get("humidity-to-location").unwrap();
    assert!(map.source.len() > 0);
    let df = DataFrame::new(vec![
        Series::new("dst", &map.destination), 
        Series::new("range", &map.range)]).unwrap();
    let sdf = df.sort(["dst", "range"], false, true).unwrap();
    let starts = sdf.column("dst").unwrap();
    let ranges = sdf.column("range").unwrap();
    assert_eq!(starts.len(), ranges.len());

    let mut location_ranges: Vec<(u32, u32)> = Vec::new();
    for i in 1..starts.len() {
        location_ranges.push(
            (starts.u32().expect("not u32").get(i).expect("was Null"),
             ranges.u32().expect("not u32").get(i).expect("was Null") )
        );
    }
    // now can iterate up through location range
    // for loc in 0..100 {
    let mut loc = 0;
    while lowest_location == u32::MAX {
        let mut src = loc;

        // lookup maps in reverse
        for map_name in map_sequence.iter().rev() {
            // println!("{} at {}", map_name, src);

            let map = maps.get(map_name).unwrap();
            let val = interpolate_map(&map.destination, &map.source, &map.range, src);
            if let Some(val) = val {
                src = val;
            } else {
                break;
            }
        }

        // check if seed is in seed range
        for seed_range in seed_ranges.iter() {
            if src >= seed_range.0 && src < seed_range.0 + seed_range.1 {
                println!("Found seed {} at location {}", src, loc);
                if loc < lowest_location {
                    lowest_location = loc;
                    break;
                }
            }
        }

        loc += 1;
    }

    lowest_location
}

fn lowest_seed_from_ranges(input: &str) -> u32 {
    let mut lowest_location: u32 = u32::MAX;

    let mut maps: HashMap<&str, Map<u32>> = HashMap::new();
    read_maps(input, &mut maps);

    // extract seed range from seed map
    let seed_map = maps.remove("seeds").unwrap();
    assert!(seed_map.source.len() > 0);

    let df = DataFrame::new(vec![
        Series::new("dst", seed_map.destination), 
        Series::new("range", seed_map.range)]).unwrap();
    let sdf = df.sort(["dst", "range"], false, true).unwrap();
    let seed_starts = sdf.column("dst").unwrap();
    let ranges = sdf.column("range").unwrap();
    let mut seed_ranges: Vec<(u32, u32)> = Vec::new();
    assert_eq!(seed_starts.len(), ranges.len());
    for i in 1..seed_starts.len() {
        seed_ranges.push(
            (seed_starts.u32().expect("not u32").get(i).expect("was Null"),
            ranges.u32().expect("not u32").get(i).expect("was Null"))
        );            
    }
    // seed_ranges now has tuples of start and length

    for seed_range in seed_ranges.iter() {
        println!("seed range: {} to {}", seed_range.0, seed_range.0 + seed_range.1);
        for seed in seed_range.0..seed_range.0 + seed_range.1 {
            let mut dst = seed;

            for map_name in map_sequence.iter() {
                // println!("{} at {}", map_name, dst);
                let map = maps.get(map_name).unwrap();
                let val = interpolate_map(&map.source, &map.destination, &map.range, dst);
                if let Some(val) = val {
                    dst = val;
                } else {
                    println!("No value found for {} at {}", map_name, dst);
                    break;
                }
            }
            // println!("seed {} at location {}", seed, dst);
            if dst < lowest_location {
                lowest_location = dst;
            }
        }   
    }

    lowest_location
}

fn interpolate_map(src: &Vec<u32>, dst: &Vec<u32>, range: &Vec<u32>, x: u32) -> Option<u32> {
    let val: Option<u32>; // = None;

    // build dataframe and sort
    let df = DataFrame::new(vec![
        Series::new("src", src), 
        Series::new("dst", dst), 
        Series::new("range", range)]).unwrap();
    // sort it on the source column
    let sdf = df.sort(["src", "dst", "range"], false, true).unwrap();
    
    // lookup x in the source column
    let sources = sdf.column("src").unwrap();
    // find index of value less than seed
    let mask = sources.lt_eq(x).unwrap();
    let less_df = sdf.filter(&mask).unwrap();

    let sources = less_df.column("src").unwrap();
    let destinations = less_df.column("dst").unwrap();
    let ranges = less_df.column("range").unwrap();
    let num_sources = sources.len();

    let src_start = sources.u32().expect("not u32").get(num_sources - 1).expect("was Null");
    let dst_start = destinations.u32().expect("not u32").get(num_sources - 1).expect("was Null");
    let range_start = ranges.u32().expect("not i32").get(num_sources - 1).expect("was Null");

    let offset = x - src_start;
    let increment = offset;
    // assert!( range_start >= offset );
    if offset > range_start {
        // offset is bigger than defined... 'reset' the dest to 'track' the source
        val = Some(src_start + increment);
    } else {
        val = Some(dst_start + increment);
    }

    val
}

fn main() {
    let input  = include_str!("../input.txt");
    let result = lowest_seed_from_ranges(input);
    println!("Lowest location: {}", result);

    // answer : 100165128
    // really should make a test case
}


#[cfg(test)]
mod tests {
    use super::*;

    // reworked the seed list to have a format more consistent with the other maps
    // duplicated the first number to basically make src/dest the same and move the second number to the range.
    // for part 1 tests (test for regression) -- this mapping will need to be modified back to a seed list
    // to modify, pop the first number off src and range vectors and push both onto  a seed list vector
    // can ignore dest vector
    const INPUT: &str =
            "seeds ranges: 
            79 79 14
            55 55 13

            seed-to-soil map:
            50 98 2
            52 50 48
            
            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15
            
            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4
            
            water-to-light map:
            88 18 7
            18 25 70
            
            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13
            
            temperature-to-humidity map:
            0 69 1
            1 0 69
            
            humidity-to-location map:
            60 56 37
            56 93 4";
    // input was modified to be more consistent with the other maps
    // basically 
    //  - add a word after seeds
    //  - newline after the ':'
    //  - pair up numbers and put on one line each pair -- e.g. 'x y'
    //  - duplicate the first number in each line so the line looks like 'x x y'
    // take care to use ONLY one space to separate fields


    #[test]
    fn test_lowest_location() {
        assert_eq!(lowest_location(INPUT), 35);
    }

    #[test]
    fn test_interpolate() {
        let src = vec![0, 98, 50];
        let dst = vec![0, 50, 52];
        let range = vec![u32::MAX, 2, 48];

        assert_eq!(interpolate_map(&src, &dst, &range, 79).unwrap(), 81);
        assert_eq!(interpolate_map(&src, &dst, &range, 14).unwrap(), 14);
        assert_eq!(interpolate_map(&src, &dst, &range, 55).unwrap(), 57);
        assert_eq!(interpolate_map(&src, &dst, &range, 13).unwrap(), 13);
        assert_eq!(interpolate_map(&src, &dst, &range, 98).unwrap(), 50);
        assert_eq!(interpolate_map(&src, &dst, &range, 99).unwrap(), 51);
        assert_eq!(interpolate_map(&src, &dst, &range, 97).unwrap(), 99);
        assert_eq!(interpolate_map(&src, &dst, &range, 0).unwrap(), 0);
        assert_eq!(interpolate_map(&src, &dst, &range, 49).unwrap(), 49);
        assert_eq!(interpolate_map(&src, &dst, &range, 50).unwrap(), 52);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        assert_eq!(lowest_location(input), 178159714);
    }

    #[test]
    fn test_seed_ranges() {
        assert_eq!(lowest_seed_from_ranges(INPUT), 46);
    }

    #[test]
    fn test_reverse_map() {
        let mut maps: HashMap<&str, Map<u32>> = HashMap::new();
        read_maps(INPUT, &mut maps);

        // can i run the interpolation backwards?
        let map = maps.get("humidity-to-location").unwrap();
        let src = &map.destination;
        let dst = &map.source;   
        let range = &map.range;

        // location 46 in the humidity-to-location dest map should be 46 -- humidity
        assert_eq!(interpolate_map(&src, &dst, &range, 46).unwrap(), 46);
        
        // humidity 46 - back to temp 45
        let map = maps.get("temperature-to-humidity").unwrap();
        assert_eq!(interpolate_map(&map.destination, &map.source, &map.range, 46).unwrap(), 45);
    
        // temp 45 - back to light 77
        let map = maps.get("light-to-temperature").unwrap();
        assert_eq!(interpolate_map(&map.destination, &map.source, &map.range, 45).unwrap(), 77);

        // light 77 - back to water 84
        let map = maps.get("water-to-light").unwrap();
        assert_eq!(interpolate_map(&map.destination, &map.source, &map.range, 77).unwrap(), 84);

        // water 84 - back to fertilizer 84
        let map = maps.get("fertilizer-to-water").unwrap();
        assert_eq!(interpolate_map(&map.destination, &map.source, &map.range, 84).unwrap(), 84);
        
        // fertilizer 84 - back to soil 84
        let map = maps.get("soil-to-fertilizer").unwrap();
        assert_eq!(interpolate_map(&map.destination, &map.source, &map.range, 84).unwrap(), 84);

        // soil 84 - back to seed 79
        let map = maps.get("seed-to-soil").unwrap();
        let seed = interpolate_map(&map.destination, &map.source, &map.range, 84).unwrap();
        assert_eq!(seed, 82);
    }

    #[test]
    fn test_reverse() {
        assert!(lowest_with_seed(INPUT) == 46);
    }

}
