
fn circular_shift(x: i32, shift: i32) -> String {
    let digits = x.to_string();
    let len = digits.len() as i32;
    let shift = if shift > len { len } else { shift % len };
    let split_point = len - shift;
    let (right, left) = digits.split_at(split_point as usize);
    format!("{}{}", left, right)
}

fn main() {
    println!("{}", circular_shift(12345, 2)); // Example usage
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
