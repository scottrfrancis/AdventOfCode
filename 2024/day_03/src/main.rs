use regex::Regex;

const OPCODES_RE: [&str; 1] = [r"mul\(\d+,\d+\)"];
const PART2_RE: [&str; 3] = [r"mul\(\d+,\d+\)", r"do\(\)", r"don't\(\)"];

fn parse_input<'a>(input: &'a str, opcodes_re: Vec<&str>) -> Vec<&'a str> {
    let mut matches = Vec::new();

    for op_re in opcodes_re.iter() {
        let re = Regex::new(op_re).unwrap();

        for cap in re.captures_iter(input) {
            let m = cap.get(0).unwrap();
            matches.push(m);
        }
    }

    // sort the matches by start index
    matches.sort_by(|a, b| a.start().cmp(&b.start()));
    // copy matches into the sequence as strings
    matches.iter().map(|m| m.as_str()).collect()
}

fn mul(operands: &Vec<i32>) -> i32 {
    operands[0] * operands[1]
}

fn execute(ops: Vec<&str>) -> i32 {
    let mut result = 0;
    let mut mul_scale_enable = 1;
    for op in ops {
        // extract opcode and operands - the opcode is the bit before the first '('
        let opcode = op.split('(').next().unwrap();
        let mut operands = vec![];
        let arg = op.split('(').nth(1).unwrap().split(')').next();
        if arg.is_some() && arg.unwrap().len() > 0 {
            operands = arg.unwrap().split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        }
    
        // execute the operation
        match opcode {
            "do" => mul_scale_enable = 1,
            "don't" => mul_scale_enable = 0,
            "mul" => result += mul(&operands)*mul_scale_enable,
            _ => (),
        };
    }

    result
}

fn main() {
    let input = include_str!("../input.txt");
    let ops = parse_input(input,OPCODES_RE.to_vec());
    let result = execute(ops);
    println!("Result: {}", result);

    // part 2
    let ops = parse_input(input, PART2_RE.to_vec());
    let result = execute(ops);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = execute(parse_input(include_str!("../input.txt"), PART2_RE.to_vec()));
        assert_eq!(result, 108830766)
    }

    #[test]
    fn test_enables() {
        let ops = parse_input(SAMPLE, PART2_RE.to_vec());
        assert_eq!(ops.len(), 6);

        let result = execute(ops);
        assert_eq!(result, 48);
    }

    #[test]
    fn test_parse_multi() {
        let ops = parse_input(SAMPLE, PART2_RE.to_vec());

        assert_eq!(ops.len(), 6);
    }

    #[test]
    fn test_part1() {
        let result = execute(parse_input(include_str!("../input.txt"), OPCODES_RE.to_vec()));
        assert_eq!(result, 165225049);
    }

    const SAMPLE: &str = 
        // "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_sample() {
        let ops = parse_input(SAMPLE, OPCODES_RE.to_vec());
        assert_eq!(ops.len(), 4);

        let result = execute(ops);
        assert_eq!(result, 161);
    }
}