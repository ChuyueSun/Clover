
fn closest_integer(value: &str) -> i32 {
    let float_value = value.parse::<f64>().unwrap_or(0.0);
    let ceil_value = float_value.ceil();
    let floor_value = float_value.floor();
    
    if (float_value - floor_value) == (ceil_value - float_value) {
        if float_value > 0.0 {
            ceil_value as i32
        } else {
            floor_value as i32
        }
    } else {
        float_value.round() as i32
    }
}

fn main() {
    // Example usage
    println!("{}", closest_integer("14.5"));  // Outputs 15
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
