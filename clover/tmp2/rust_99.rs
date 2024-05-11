
fn closest_integer(value: &str) -> i32 {
    let num = value.parse::<f64>().unwrap();
    if num.fract() == 0.5 || num.fract() == -0.5 {
        // Round away from zero
        if num > 0.0 { num.ceil() } else { num.floor() }
    } else {
        // Use standard rounding
        num.round()
    } as i32
}

fn main() {
    // Example usage:
    println!("{}", closest_integer("14.5")); // Should print 15
    println!("{}", closest_integer("-14.5")); // Should print -15
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
