
fn closest_integer(value: &str) -> i32 {
    let float_value: f64 = value.parse().unwrap();
    if float_value.fract() == 0.5 || float_value.fract() == -0.5 {
        if float_value > 0.0 {
            float_value.ceil() as i32
        } else {
            float_value.floor() as i32
        }
    } else {
        float_value.round() as i32
    }
}

fn main() {
    // Testing the function with some examples
    println!("{}", closest_integer("14.5"));   // Should output 15
    println!("{}", closest_integer("-14.5"));  // Should output -15
    println!("{}", closest_integer("3.3"));    // Should output 3
    println!("{}", closest_integer("-3.3"));   // Should output -3
    println!("{}", closest_integer("0"));      // Should output 0
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
