
fn below_zero(operations: Vec<i32>) -> bool {
    let mut balance = 0;
    for op in operations {
        balance += op;
        if balance < 0 {
            return true;
        }
    }
    false
}

fn main() {
    // Example usage:
    // Account starts at zero balance.
    // Operations are: deposit 100, withdraw 50, withdraw 100.
    let operations = vec![100, -50, -100];
    println!("Does the balance go below zero? {}", below_zero(operations));
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
