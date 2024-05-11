
/// Sums numbers from 1 to n.
///
/// # Arguments
///
/// * `n` - The upper bound integer to sum up to, inclusively.
///
/// # Returns
///
/// The sum of all integers from 1 to `n`.
fn sum_to_n(n: i32) -> i32 {
    (1..=n).fold(0, |sum, num| sum + num)
}

fn main() {
    let n = 10;
    println!("The sum of numbers from 1 to {} is {}", n, sum_to_n(n));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_to_n() {
        assert!(sum_to_n(1) == 1);
        assert!(sum_to_n(6) == 21);
        assert!(sum_to_n(11) == 66);
        assert!(sum_to_n(30) == 465);
        assert!(sum_to_n(100) == 5050);
    }

}
