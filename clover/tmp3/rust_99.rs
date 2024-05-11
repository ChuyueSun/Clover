
fn closest_integer(value: &str) -> i32 {
    match value.parse::<f64>() {
        Ok(num) => {
            if num.fract() == 0.5 || num.fract() == -0.5 {
                if num.is_sign_positive() {
                    num.ceil() as i32
                } else {
                    num.floor() as i32
                }
            } else {
                num.round() as i32
            }
        },
        Err(_) => 0,
    }
}

fn main() {
    // Example usage:
    println!("{}", closest_integer("14.5")); // should return 15
    println!("{}", closest_integer("-14.5")); // should return -15
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
