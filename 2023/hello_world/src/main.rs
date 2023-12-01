fn message() -> String {
    "Hello, world!".to_string()
}


fn main() {
    println!("{}", message());
}
 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message() {
        assert_eq!(message(), "Hello, world!");
    }
}
