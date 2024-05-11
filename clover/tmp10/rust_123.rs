
fn get_odd_collatz(n: i32) -> Vec<i32> {
    let mut collatz_number = n;
    let mut odd_numbers = vec![];

    while collatz_number != 1 {
        if collatz_number % 2 != 0 {
            odd_numbers.push(collatz_number);
        }
        if collatz_number % 2 == 0 {
            collatz_number /= 2;
        } else {
            collatz_number = 3 * collatz_number + 1;
        }
    }

    odd_numbers.push(1); // Always includes the last number 1, since it's also odd
    odd_numbers.sort_unstable();
    odd_numbers
}

fn main() {
    // Example usage:
    let odd_collatz = get_odd_collatz(7);
    println!("{:?}", odd_collatz); // Should print [1, 5, 7, 11, 13]
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
