
fn string_sequence(n: i32) -> String {
    (0..=n).map(|i| i.to_string()).collect::<Vec<_>>().join(" ")
}

fn main() {
    // Example usage:
    let result = string_sequence(5);
    println!("{}", result); // Should print "0 1 2 3 4 5"
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_string_sequence() {
        assert!(string_sequence(0) == "0".to_string());
        assert!(string_sequence(3) == "0 1 2 3".to_string());
        assert!(string_sequence(10) == "0 1 2 3 4 5 6 7 8 9 10".to_string());
    }

}
