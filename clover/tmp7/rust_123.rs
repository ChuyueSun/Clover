
fn get_odd_collatz(n: i32) -> Vec<i32> {
    let mut vec = Vec::new();
    let mut number = n;

    while number != 1 {
        if number % 2 != 0 {
            vec.push(number);
        }
        // Apply the Collatz sequence rules.
        number = if number % 2 == 0 {
            number / 2
        } else {
            3 * number + 1
        };
    }
    // Add the 1 at the end, since it's an odd number and part of the sequence.
    vec.push(1);
    // The vec will contain odd numbers in their sequence order, to sort them we can just use sort method.
    vec.sort_unstable();
    vec
}

fn main() {
    // Example usage:
    let odd_collatz = get_odd_collatz(19);
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
