
/// Add two numbers x and y
///
/// # Arguments
///
/// * `x` - An i32 integer to be added.
/// * `y` - Another i32 integer to be added.
///
/// # Returns
///
/// * The sum of x and y as i32.
fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    // Example usage:
    let result = add(5, 3);
    println!("The sum is: {}", result);
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
