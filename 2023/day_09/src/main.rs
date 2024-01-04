/*** Day 9 
 * 
 * Derivatives
 *  
 */
// import polars series
use polars::prelude::*;

// build Polars series for each line of input
fn parse_input(input: &str) -> Vec<Series> {
    input.lines().map(|line| {
        let arr = line.trim().split(' ').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let s: Series = arr.iter().collect();
        s
    }).collect()
}

fn take_derivative(series: &Series) -> Series {
    let len = Some(series.len() - 1);
    let mut out_series = series.tail(len).clone();
    out_series = out_series - series.head(len);
    out_series
}

fn extend_series(series: &Series) -> (i32, i32) {
    let len = series.len();
    let zeros = Series::new("", vec![0; len]);
    
    let mut derivatives: Vec<Series> = Vec::new();
    let mut ds = series.clone();
    while ds != zeros.head(Some(ds.len())) {
        ds = take_derivative(&ds);
        derivatives.push(ds.clone());
    }
    
    // now expand the derivatives
    let last = derivatives.pop().unwrap();
    assert!(last == zeros.head(Some(last.len())));
    let (mut last_d, mut first_d) = (0, 0);
    while derivatives.len() > 0 {
        ds = derivatives.pop().unwrap();

        // extend last series
        // get the last value of ds
        // add it to last_d
        let x = ds.tail(Some(1)).i32().unwrap().get(0).unwrap();
        last_d += x;
        let y = ds.head(Some(1)).i32().unwrap().get(0).unwrap();
        first_d = y - first_d;
        // integrate from Last to ds
    }
    
    let x = series.tail(Some(1)).i32().unwrap().get(0).unwrap();
    let y = series.head(Some(1)).i32().unwrap().get(0).unwrap();
    (y - first_d, last_d + x)
}


fn main() {
    println!("Part 1");
    let input  = include_str!("../input.txt");
    let in_series = parse_input(input);
    let (mut sum_next, mut sum_prev) = (0, 0);
    for s in in_series {
        println!("s: {:?}", s);
        let (prev, next) = extend_series(&s);
        println!("next: {:?}, prev: {:?}", next, prev);
        sum_next += next;
        sum_prev += prev;
    }    
    println!("sum: {:?}", sum_next);

    println!("Part 2");
    println!("sum: {:?}", sum_prev);
    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input  = include_str!("../input.txt");
        let in_series = parse_input(input);
        let mut sum = 0;
        for s in in_series {
            println!("s: {:?}", s);
            let (prev, _) = extend_series(&s);
            println!("prev: {:?}", prev);
            sum += prev;
        }    
        println!("sum: {:?}", sum);

        assert_eq!(sum, 884);   
    
    }

    #[test]
    fn test_part1() {
        let input  = include_str!("../input.txt");
        let in_series = parse_input(input);
        let mut sum = 0;
        for s in in_series {
            println!("s: {:?}", s);
            let (_, next) = extend_series(&s);
            println!("next: {:?}", next);
            sum += next;
        }    
        println!("sum: {:?}", sum);

        assert_eq!(sum, 1974913025);   
    }

    #[test]
    fn test_sample() {
        let in_series = parse_input(INPUT);
        let (mut sum_next, mut sum_prev) = (0, 0);
        for s in in_series {
            println!("s: {:?}", s);
            let (prev, next) = extend_series(&s);
            println!("next: {:?} prev: {:?}", next, prev);
            sum_next += next;
            sum_prev += prev;
        }

        assert_eq!(sum_next, 114);
        assert_eq!(sum_prev, 2);
    }

    #[test]
    fn test_extend_series() {
        let in_series: Series = [1, 3, 6, 10, 15, 21].iter().collect();
        let (prev, next) = extend_series(&in_series);

        assert_eq!(next, 28);
        assert_eq!(prev, 0);
    }

    #[test]
    fn test_take_derivative() {
        let in_series: Series = [0, 3, 6, 9, 12, 15].iter().collect();

        let out_series = take_derivative(&in_series);
        let result = out_series.i32().unwrap().into_no_null_iter().collect::<Vec<i32>>();
        assert_eq!(result, vec![3;5]);
    }

    #[test]
    fn test_parse_series() {
        let in_series = parse_input(INPUT);

        assert_eq!(in_series.len(), 3);
    }

    const INPUT: &str =
        "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";
        
}

