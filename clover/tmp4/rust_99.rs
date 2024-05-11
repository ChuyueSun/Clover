
fn closest_integer(value: &str) -> i32 {
    let parsed = value.parse::<f64>().unwrap();  // Assuming the string is always a valid number
    if parsed > 0.0 {
        (parsed + 0.5).floor() as i32
    } else {
        (parsed - 0.5).ceil() as i32
    }
}

fn main() {
    // Example usage:
    println!("{}", closest_integer("14.5"));  // Outputs: 15
    println!("{}", closest_integer("-14.5")); // Outputs: -15
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_closest_integer() {
        assert!(closest_integer("10") == 10);
        assert!(closest_integer("14.5") == 15);
        assert!(closest_integer("-15.5") == -16);
        assert!(closest_integer("15.3") == 15);
        assert!(closest_integer("0") == 0);
    }

}
