
fn get_odd_collatz(n: i32) -> Vec<i32> {
    let mut collatz = n;
    let mut odd_collatz_numbers = Vec::new();

    while collatz != 1 {
        if collatz % 2 != 0 {
            odd_collatz_numbers.push(collatz);
        }
        collatz = if collatz % 2 == 0 {
            collatz / 2
        } else {
            3 * collatz + 1
        };
    }
    odd_collatz_numbers.push(1); // Include 1 as specified by the note
    odd_collatz_numbers.sort_unstable();
    odd_collatz_numbers
}

fn main() {
    let n = 13; // Example usage
    let sorted_odd_collatz = get_odd_collatz(n);
    println!("{:?}", sorted_odd_collatz);
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
