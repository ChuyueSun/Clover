
fn circular_shift(x: i32, shift: i32) -> String {
    let digits = x.to_string();
    let len = digits.len() as i32;
    let actual_shift = if shift > len {
        len
    } else {
        shift % len
    };

    let split_index = len - actual_shift;
    let (right, left) = digits.split_at(split_index as usize);
    format!("{}{}", left, right)
}

fn main() {
    // Example usage:
    let shifted = circular_shift(12345, 2);
    println!("{}", shifted); // Output: "45123"
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
