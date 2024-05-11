
/// Adds two numbers, `x` and `y`.
///
/// # Arguments
///
/// * `x` - The first integer.
/// * `y` - The second integer.
///
/// # Examples
///
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    // Example usage:
    let sum = add(10, 15);
    println!("The sum of 10 and 15 is {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert!(add(0, 1) == 1);
        assert!(add(1, 0) == 1);
        assert!(add(2, 3) == 5);
        assert!(add(5, 7) == 12);
        assert!(add(7, 5) == 12);
        for _ in 0..100 {
            let mut rng = rand::thread_rng();
            let mut x: i32 = rng.gen();
            x = x % 1000;
            let mut y: i32 = rng.gen();
            y = y % 1000;

            assert!(add(x, y) == x + y);
        }
    }

}
