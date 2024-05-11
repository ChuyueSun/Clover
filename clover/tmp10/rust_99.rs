
fn closest_integer(value: &str) -> i32 {
    let number = value.parse::<f64>().unwrap();
    if number.fract() == 0.5 || number.fract() == -0.5 {
        number.signum() * (number.abs().floor() + 1.0) as i32
    } else {
        number.round() as i32
    }
}

fn main() {
    // Example usages:
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
