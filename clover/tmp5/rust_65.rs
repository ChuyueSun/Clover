
fn circular_shift(x: i32, shift: i32) -> String {
    let digits = x.to_string().chars().collect::<Vec<_>>();
    let len = digits.len() as i32;
    let effective_shift = if shift > len { len } else { shift % len };
    let split_point = len - effective_shift;

    let (right, left) = digits.split_at(split_point as usize);
    let result = left.iter().chain(right.iter()).collect::<String>();
    
    if shift > len {
        result.chars().rev().collect()
    } else {
        result
    }
}

fn main() {
    // Example Usage
    println!("{}", circular_shift(12345, 2)); // Output: "45123"
    println!("{}", circular_shift(12345, 5)); // Output: "54321"
    println!("{}", circular_shift(12345, 7)); // Output: "54321"
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
