
fn below_zero(operations: Vec<i32>) -> bool {
    let mut balance: i32 = 0;
    for operation in operations {
        balance += operation;
        if balance < 0 {
            return true;
        }
    }
    false
}

fn main() {
    // Example usage:
    let operations = vec![100, -50, -60, 10]; // This should return false.
    println!("Account goes below zero: {}", below_zero(operations));

    let operations2 = vec![100, -150, 50, -10]; // This should return true.
    println!("Account goes below zero: {}", below_zero(operations2));
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_below_zero() {
        assert_eq!(below_zero(vec![]), false);
        assert_eq!(below_zero(vec![1, 2, -3, 1, 2, -3]), false);
        assert_eq!(below_zero(vec![1, 2, -4, 5, 6]), true);
        assert_eq!(below_zero(vec![1, -1, 2, -2, 5, -5, 4, -4]), false);
        assert_eq!(below_zero(vec![1, -1, 2, -2, 5, -5, 4, -5]), true);
        assert_eq!(below_zero(vec![1, -2, 2, -2, 5, -5, 4, -4]), true);
    }

}
