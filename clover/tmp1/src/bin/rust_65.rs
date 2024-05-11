
fn circular_shift(x: i32, shift: i32) -> String {
    let digits = x.abs().to_string(); // Convert the number to string to get the digits
    let len = digits.len() as i32; // Find the number of digits
    
    // Adjust shift to be within the range of 0..len
    let shift = if shift > len { len } else { shift % len }; 

    // Simple case where no shift or full reverse is needed
    if shift == 0 {
        return digits;
    } else if shift == len {
        return digits.chars().rev().collect();
    }

    // Calculate the effective shift
    let shift = (len - shift).rem_euclid(len) as usize;

    // Perform the circular shift
    let (start, end) = digits.split_at(shift);
    format!("{}{}", end, start)
}

// Example usage:
// let result = circular_shift(12345, 2);
// println!("{}", result); //Output would be: "34125"

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
