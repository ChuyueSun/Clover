
fn get_odd_collatz(n: i32) -> Vec<i32> {
    let mut sequence = Vec::new();
    let mut number = n;

    while number != 1 {
        if number % 2 != 0 {
            sequence.push(number);
        }
        number = if number % 2 == 0 {
            number / 2
        } else {
            3 * number + 1
        };
    }
    sequence.push(1); // Include 1 as per the note.
    sequence.sort_unstable();
    sequence
}

fn main() {
    // Example usage:
    let n = 13;
    let collatz_sequence = get_odd_collatz(n);
    println!("{:?}", collatz_sequence);
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
