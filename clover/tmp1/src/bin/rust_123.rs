
fn get_odd_collatz(n: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut current = n;

    while current != 1 {
        if current % 2 != 0 {
            result.push(current);
        }
        current = if current % 2 == 0 {
            current / 2
        } else {
            3 * current + 1
        }
    }

    result.push(1); // Collatz(1) is [1]
    result.sort_unstable();
    result
}

fn main() {
    // Example usage:
    let result = get_odd_collatz(12);
    println!("{:?}", result);
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
