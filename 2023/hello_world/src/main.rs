
fn add_two(y: i32) -> impl Fn(i32) -> i32 {
    let add_one = move |x| {
        x + y
    };
    
    add_one
}


fn main() {
    let adder = add_two(5);
    let z = adder(2);
    print!("{}", z);
}


fn message() -> String {
    "Hello, world!".to_string()
}


// fn main() {
//     println!("{}", message());
// }
 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message() {
        assert_eq!(message(), "Hello, world!");
    }
}
