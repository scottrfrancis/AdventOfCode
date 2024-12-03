use polars::prelude::*; // if the crate polars-core is used directly
// use polars_io::prelude::*;
use polars::frame::DataFrame;
use regex::Regex;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

// read input string (not filename) to a polars dataframe
fn read_input(input: &str) -> DataFrame {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    // use regex to split input into columns -- format is number spaces number
    let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();

    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        list1.push(caps.get(1).unwrap().as_str().parse::<i32>().unwrap());
        list2.push(caps.get(2).unwrap().as_str().parse::<i32>().unwrap());
    }

    let sx = Column::new("x".into(), list1);
    let sy = Column::new("y".into(), list2);

    let df = DataFrame::new(vec![sx, sy]).unwrap();

    df
}

fn series_to_min_heap(series: &Series) -> BinaryHeap<Reverse<i32>> {
    let mut heap = BinaryHeap::new();

    for value in series.iter() {
        if let AnyValue::Int32(v) = value {
            heap.push(Reverse(v));
        }
    }

    heap
}

fn column_name_to_min_heap(df: &DataFrame, column_name: &str) -> BinaryHeap<Reverse<i32>> {
    let binding = df.column(column_name).unwrap().clone();
    let series = binding.as_series().unwrap();
    
    series_to_min_heap(&series)
}

fn sum_distances(input: &str) -> i32 {
    let mut sum = 0;

    let df = read_input(input);
    let mut x_min_heap = column_name_to_min_heap(&df, "x");
    let mut y_min_heap = column_name_to_min_heap(&df, "y");

    while x_min_heap.len() > 0 {
        let x = x_min_heap.pop().unwrap().0;
        let y = y_min_heap.pop().unwrap().0;

        let distance = (x - y).abs();
        sum += distance;
    }

    sum
}

fn main() {
    // read input file
    let input = include_str!("../input.txt");
    let sum = sum_distances(input);

    println!("Sum of calibration values: {}", sum);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        let sum = sum_distances(input);

        assert_eq!(sum, 1879048);
    }

    const INPUT: &str = 
       "3   4
        4   3
        2   5
        1   3
        3   9
        3   3";

    #[test]
    fn test_sample_sum() {

        assert_eq!(sum_distances(INPUT), 11);
    }

    #[test]
    fn test_min_heap() {
        let df = read_input(INPUT);

        let mut x_min_heap = column_name_to_min_heap(&df, "x");

        assert_eq!(x_min_heap.len(), 6);
        assert_eq!(x_min_heap.pop(), Some(Reverse(1)));
        assert_eq!(x_min_heap.pop(), Some(Reverse(2)));
        assert_eq!(x_min_heap.pop(), Some(Reverse(3)));
        assert_eq!(x_min_heap.pop(), Some(Reverse(3)));
        assert_eq!(x_min_heap.pop(), Some(Reverse(3)));
        assert_eq!(x_min_heap.pop(), Some(Reverse(4)));
        assert_eq!(x_min_heap.len(), 0);

        let by = df.column("y").unwrap().clone();
        let y = by.as_series().unwrap();
        let y_min_heap = series_to_min_heap(&y);

        assert_eq!(y_min_heap.len(), 6);
    }

    #[test]
    fn test_read_input() {
        let df = read_input(INPUT);

        assert_eq!(df.shape(), (6, 2));
    }
    
}
