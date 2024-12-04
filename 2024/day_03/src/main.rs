use regex::Regex;


fn parse_input(input: &str) -> Vec<&str> {
    let mut ops = Vec::new();
    // find proper mul operations `mul(x,y)` where x and y are integers with any number of digits
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    for cap in re.captures_iter(input) {
        ops.push(cap.get(0).unwrap().as_str());
    }
 
    ops
}

fn mul(operands: &Vec<i32>) -> i32 {
    operands[0] * operands[1]
}

fn execute(ops: Vec<&str>) -> i32 {
    let mut result = 0;
    for op in ops {
        // extract opcode and operands - the opcode is the bit before the first '('
        let opcode = op.split('(').next().unwrap();
        let operands = op.split('(').nth(1).unwrap().split(')').next().unwrap()
            .split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    
        // execute the operation
        result += match opcode {
            "mul" => mul(&operands),
            _ => 0,
        };
    }

    result
}

fn main() {
    let input = include_str!("../input.txt");
    let ops = parse_input(input);
    let result = execute(ops);
    println!("Result: {}", result);

}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = 
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_sample() {
        let ops = parse_input(SAMPLE);
        assert_eq!(ops.len(), 4);

        let result = execute(ops);
        assert_eq!(result, 161);
    }
}