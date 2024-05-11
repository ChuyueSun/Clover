
fn closest_integer(value: &str) -> i64 {
    let float_value = value.parse::<f64>().expect("Invalid input");
    if float_value.fract() == 0.5 || float_value.fract() == -0.5 {
        float_value.signum() * (float_value.abs() + 0.5).floor()
    } else {
        float_value.round()
    } as i64
}

fn main() {
    // Example usage:
    println!("{}", closest_integer("14.5")); // Outputs 15
    println!("{}", closest_integer("-14.5")); // Outputs -15
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
