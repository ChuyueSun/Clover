
fn closest_integer(value: &str) -> Result<i32, std::num::ParseFloatError> {
    let num: f64 = value.parse()?;
    let floor = num.floor() as i32;
    let ceil = num.ceil() as i32;
    
    // Calculate distance of the number from the floor and ceiling values
    let dist_from_floor = num - floor as f64;
    let dist_from_ceil = ceil as f64 - num;
    
    // If the number is exactly in the middle, round away from zero
    if (dist_from_floor - dist_from_ceil).abs() < f64::EPSILON {
        if num.is_sign_positive() {
            Ok(ceil)
        } else {
            Ok(floor)
        }
    } else {
        // Otherwise, return the nearest integer
        Ok(num.round() as i32)
    }
}

fn main() {
    // Example usage:
    let result = closest_integer("14.5").unwrap();
    println!("Closest integer: {}", result); // Should print 15

    let result = closest_integer("-14.5").unwrap();
    println!("Closest integer: {}", result); // Should print -15
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
