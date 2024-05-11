
fn circular_shift(x: i32, shift: i32) -> String {
    let x_str = x.to_string();
    let num_digits = x_str.len() as i32;
    let effective_shift = if shift > num_digits {
        num_digits
    } else {
        shift % num_digits
    };

    let (left, right) = if effective_shift == num_digits {
        x_str.split_at(0)
    } else {
        x_str.split_at((num_digits - effective_shift) as usize)
    };
    
    format!("{}{}", right, left)
}

fn main() {
    // Example usage:
    let result = circular_shift(12345, 2);
    println!("{}", result); // Expected: 45123
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_circular_shift() {
        assert!(circular_shift(100, 2) == "001");
        assert!(circular_shift(12, 8) == "12");
        // original test   asert (circular_shift(97, 8) == "79"); DATASET ERROR
        assert!(circular_shift(97, 8) == "97");
        assert!(circular_shift(12, 1) == "21");
        assert!(circular_shift(11, 101) == "11");
    }

}
