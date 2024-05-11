
fn circular_shift(x: i32, shift: i32) -> String {
    let mut digits = x.to_string().chars().collect::<Vec<char>>();
    let len = digits.len() as i32;
    let effective_shift = if shift > len { len } else { shift % len };

    digits.rotate_right(effective_shift as usize);
    digits.into_iter().collect()
}

fn main() {
    // Example usage
    println!("{}", circular_shift(12345, 2)); // Should print "45123"
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
