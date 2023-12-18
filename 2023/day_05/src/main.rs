/*** Day 5
 * 
 */

use polars::prelude::*; // if the crate polars-core is used directly
use std::{collections::{HashMap}, vec};


#[derive(Debug)]
struct Map<T> {
    source: Vec<T>,
    destination: Vec<T>,
    range: Vec<T>,
}

fn lowest_location(input: &str) -> u32 {
    let mut lowest_location = u32::MAX;

    // parse input into 
    // - list of seeds to plan
    let mut lines = input.lines();
    // first section (seeds) is special -- it's all on one line

    let line = lines.next().unwrap().trim();
    let seeds = line.split(": ").nth(1).unwrap().split(" ").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    // each section is separated by a blank line.
    let line = lines.next().unwrap().trim();
    assert!(line.is_empty());

    let mut maps: HashMap<&str, Map<u32>> = HashMap::new();

    // after that, each of the map sections are similar structure
    // let maps: HashMap<&str, BTreeMap<>> = HashMap::new();
    // the map name will be the key and the parsed map will be the value.
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
    println!("{:?}\n", maps);

    // calculate location
    let map_sequence = vec![
        "seed-to-soil", 
        "soil-to-fertilizer", 
        "fertilizer-to-water", 
        "water-to-light", 
        "light-to-temperature", 
        "temperature-to-humidity", 
        "humidity-to-location"
    ];
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

fn interpolate_map(src: &Vec<u32>, dst: &Vec<u32>, range: &Vec<u32>, x: u32) -> Option<u32> {
    let mut val: Option<u32>; // = None;

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
    let result = lowest_location(input);
    println!("Lowest location: {}", result);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lowest_location() {
        let input =
            "seeds: 79 14 55 13

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

        assert_eq!(lowest_location(input), 35);
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

}
