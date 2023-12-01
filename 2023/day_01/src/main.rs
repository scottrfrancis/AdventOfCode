use phf::{phf_map}; //, Map};

// make a map of spelled out numbers to their digit values
static NUMBERS: phf::Map<&'static str, u8> = phf_map! {
    "zero" => 0,
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9,
};


fn sum_calibration_values(calibration_values: &str) -> u32 {
    let mut sum: u32 = 0;


    // let mut reversed_numbers: HashMap<String, u8> = HashMap::new();
    // for (k, v) in &NUMBERS {
    //     let kr = k.chars().rev().collect::<String>();
    //     reversed_numbers.insert(kr, *v);
    // }

    // iterate over each line in the string calibration_values
    // and add the values to the sum
    for line in calibration_values.lines() {
        // strip whitespace from the line
        let line = line.trim();

        let mut first_digit: Option<u8> = None;
        // iterate over each character in the line forward to get left digit by index
        for (i, c) in line.chars().enumerate() {
            // check if the character is a digit
            if c.is_digit(10) {
                // convert the character to a digit
                let digit = c.to_digit(10).unwrap();
                // check if the digit is the first digit
                if first_digit.is_none() {
                    first_digit = Some(digit as u8);
                }
            } else {
                // check if substring starting at i is in NUMBERS
                // enumerate over the keys of NUMBERS and check each substring in line from i to the end
                for (k, v) in &NUMBERS {
                    if line[i..].starts_with(k) {
                        first_digit = Some(*v);
                        break;
                    }
                }
            }
            // if first digit is set, break out of the loop
            if first_digit.is_some() {
                break;
            }
        }

        // now iterate in reverse to get the right digit
        let mut last_digit: Option<u8> = None;
        for (i,c) in line.chars().rev().enumerate() {
            if c.is_digit(10) {
                let digit = c.to_digit(10).unwrap();
                if last_digit.is_none() {
                    last_digit = Some(digit as u8);
                } 
            } else {
                // check if substring starting at i is in NUMBERS
                // enumerate over the keys of NUMBERS and check each substring in line from i to the end
                for (k, v) in &NUMBERS {
                    if line[..line.len()-i].ends_with(k) {
                        last_digit = Some(*v);
                        break;
                    }
                }

            }

            if last_digit.is_some() {
                break;
            }
        }

        // if both are set, add them to the sum
        if last_digit.is_some() && first_digit.is_some() {
            let line_sum = (first_digit.unwrap() * 10 + last_digit.unwrap()) as u32;
            sum += line_sum;
        }
    }

    sum
}


fn main() {
    // read input file
    let input = include_str!("../input.txt");
    let sum = sum_calibration_values(input);

    println!("Sum of calibration values: {}", sum);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_calibration_values() {
        let calibration_values = 
"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let sum = sum_calibration_values(&calibration_values);
        assert_eq!(sum, 142);
    }

    #[test]
    fn test_sum_with_words() {
        let calibration_values = 
            "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen";
        let sum = sum_calibration_values(&calibration_values);
        assert_eq!(sum, 281);
    }
    
}
