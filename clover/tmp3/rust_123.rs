
fn get_odd_collatz(n: i32) -> Vec<i32> {
    let mut collatz = n;
    let mut odd_collatz_numbers = vec![];

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
    odd_collatz_numbers.push(1); // The sequence always ends with 1, which is also odd
    odd_collatz_numbers.sort_unstable();
    odd_collatz_numbers
}

fn main() {
    let n = 10;
    let odd_collatz_sequence = get_odd_collatz(n);
    println!("{:?}", odd_collatz_sequence);
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
