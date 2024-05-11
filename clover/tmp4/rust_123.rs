
fn get_odd_collatz(mut n: i32) -> Vec<i32> {
    let mut odd_collatz_numbers = Vec::new();

    while n != 1 {
        if n % 2 != 0 {
            odd_collatz_numbers.push(n);
        }
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
    }
    odd_collatz_numbers.push(1); // Adding 1 since it's always odd and at the end of the sequence.
    odd_collatz_numbers.sort_unstable(); // Sort the numbers in increasing order.
    odd_collatz_numbers
}

fn main() {
    let n = 11;
    let odd_collatz = get_odd_collatz(n);
    println!("{:?}", odd_collatz);
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
