
fn get_odd_collatz(n: i32) -> Vec<i32> {
    let mut sequence = Vec::new();
    let mut val = n;

    while val != 1 {
        if val % 2 != 0 {
            sequence.push(val);
        }
        if val % 2 == 0 {
            val /= 2;
        } else {
            val = 3 * val + 1;
        }
    }

    sequence.push(1); // Add the final value (1) to the sequence
    sequence.sort_unstable();
    sequence
}

fn main() {
    println!("{:?}", get_odd_collatz(13)); // Example usage; replace with any positive integer.
}

#[cfg(test)]
mod tests {
    use super::*;

   #[test]
    fn test_get_odd_collatz() {
        assert_eq!(get_odd_collatz(14), vec![1, 5, 7, 11, 13, 17]);
        assert_eq!(get_odd_collatz(5), vec![1, 5]);
        assert_eq!(get_odd_collatz(12), vec![1, 3, 5]);
        assert_eq!(get_odd_collatz(1), vec![1]);
    }

}
