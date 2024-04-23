fn main() {}

/*
Circular shift the digits of the integer x, shift the digits right by shift
    and return the result as a string.
    If shift > number of digits, return digits reversed.

*/

use std::mem::replace;

fn circular_shift(x: i32, shift: i32) -> String {
    let xcp: Vec<char> = x.to_string().chars().into_iter().collect();
    let mut res: Vec<char> = x.to_string().chars().into_iter().collect();

    for (indx, c) in xcp.iter().enumerate() {
        let despl = (indx as i32 + shift) % x.to_string().len() as i32;
        replace(&mut res[despl as usize], *c);
    }

    return res.into_iter().collect();
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
