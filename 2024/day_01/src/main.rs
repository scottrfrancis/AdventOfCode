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

fn column_term_frequencies(df: &DataFrame, column_name: &str) -> DataFrame {
    let binding = df.column(column_name).unwrap().clone();
    let series = binding.as_series().unwrap();

    let tf = series.value_counts(true, true, "count".into(), false).unwrap();
    // let term_bind = tf.column(column_name).unwrap().clone();
    // let terms = term_bind.as_series().unwrap();
    // let counts_bind = tf.column("count").unwrap().clone();
    // let counts = counts_bind.as_series().unwrap();
    // for (term, count) in terms.iter().zip(counts.iter()) {
    //     println!("{}: {}", term, count);
    // }    

    tf
}

fn get_term_frequency(tf: &DataFrame, term: i32) -> i32 {
    let cols = tf.get_columns();
    let terms = cols[0].as_series().unwrap();

    // find index of term in terms
    let mask = terms.equal(term).unwrap();

    let counts = cols[1].as_series().unwrap();
    // get the count at the true value of the mask
    let count = counts.filter(&mask).unwrap().sum::<i32>().unwrap();
    
    count
}

fn get_frequencies(tf: &DataFrame, terms: &Series) -> Series {
    let mut weights = Vec::new();

    for term in terms.iter() {
        if let AnyValue::Int32(t) = term {
            let f = get_term_frequency(tf, t);
            weights.push(f);
        }
    }

    Series::new("weights".into(), weights)
}

fn get_similarity_score(df: &DataFrame, x_col: &str, y_col: &str) -> i32 {
    let tf = column_term_frequencies(&df, y_col);
    let terms_bind = df.column(x_col).unwrap().clone();
    let terms = terms_bind.as_series().unwrap();

    let weights = get_frequencies(&tf, &terms);

    // take dot product of weights and terms
    let sum: i32 = terms.i32()
        .unwrap()
        .into_iter()
        .zip(weights.i32().unwrap().into_iter())
        .map(|(x, w)| x.unwrap() * w.unwrap())
        .sum();

    sum
}

fn main() {
    // read input file
    let input = include_str!("../input.txt");
    // part 1
    let sum = sum_distances(input);
    println!("Sum of calibration values: {}", sum);

    // part 2
    let df = read_input(input);
    let sum = get_similarity_score(&df, "x", "y");
    println!("Similarity score: {}", sum);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = include_str!("../input.txt");
        let df = read_input(input);
        let sum = get_similarity_score(&df, "x", "y");

        assert_eq!(sum, 21024792);
    }

    #[test]
    fn test_similarity_score() {
        let df = read_input(INPUT);
        let tf = column_term_frequencies(&df, "y");

        let terms_bind = df.column("x").unwrap().clone();
        let terms = terms_bind.as_series().unwrap();

        let weights = get_frequencies(&tf, &terms);

        assert_eq!(weights.len(), 6);
        for i in 0..6 {
            let x: i32 = match terms.get(i).unwrap() {
                AnyValue::Int32(v) => v,
                _ => panic!("Unexpected data type"),
            };
            let w: i32 = match weights.get(i).unwrap() {
                AnyValue::Int32(v) => v,
                _ => panic!("Unexpected data type"),
            };

            println!("x: {}, w: {}", x, w);
            match x {
                3 => assert_eq!(w, 3),
                4 => assert_eq!(w, 1),
                5 => assert_eq!(w, 1),
                9 => assert_eq!(w, 1),
                _ => assert_eq!(w, 0),
            }
        }

        // take dot product of weights and terms
        let sum: i32 = terms.i32()
            .unwrap()
            .into_iter()
            .zip(weights.i32().unwrap().into_iter())
            .map(|(x, w)| x.unwrap() * w.unwrap())
            .sum();
        assert_eq!(sum, 31);
    }

    #[test]
    fn test_term_frequency() {
        // let input = include_str!("../input.txt");
        let df = read_input(INPUT);

        let tf = column_term_frequencies(&df, "y");

        assert_eq!(tf.shape(), (4, 2));
    
        assert_eq!(get_term_frequency(&tf, 3), 3);
        for x in [ 4, 5, 9].iter() {
            let f = get_term_frequency(&tf, *x);
            assert_eq!(f, 1);
        }
        
        assert_eq!(get_term_frequency(&tf, 2), 0);
    }
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
