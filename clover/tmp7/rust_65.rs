
fn circular_shift(x: i32, shift: i32) -> String {
    let digits = x.to_string();
    let len = digits.len() as i32;
    
    // If shift > number of digits, return digits reversed.
    if shift > len {
        return digits.chars().rev().collect();
    }
    
    // Shift the digits right by 'shift'.
    let shift = shift as usize % len as usize;
    let (right, left) = digits.split_at(len as usize - shift);
    format!("{}{}", left, right)
}

fn main() {
    // Example Usage:
    println!("{}", circular_shift(12345, 2)); // Outputs "34125"
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
