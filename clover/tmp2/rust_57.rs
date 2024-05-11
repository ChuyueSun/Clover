
fn monotonic(l: Vec<i32>) -> bool {
    if l.is_empty() || l.len() == 1 {
        return true;
    }
    
    let mut increasing = false;
    let mut decreasing = false;

    for i in 1..l.len() {
        if l[i] > l[i - 1] {
            increasing = true;
        } else if l[i] < l[i - 1] {
            decreasing = true;
        }
    }

    !(increasing && decreasing)
}

fn main() {
    // Test cases
    println!("{}", monotonic(vec![1, 2, 3, 4, 5])); // Should print true for increasing
    println!("{}", monotonic(vec![5, 4, 3, 2, 1])); // Should print true for decreasing
    println!("{}", monotonic(vec![1, 2, 2, 3]));    // Should print true for non-strictly increasing
    println!("{}", monotonic(vec![1, 2, 1]));       // Should print false
    println!("{}", monotonic(Vec::<i32>::new()));   // Should print true for empty list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monotonic() {
        assert!(monotonic(vec![1, 2, 4, 10]) == true);
        assert!(monotonic(vec![1, 2, 4, 20]) == true);
        assert!(monotonic(vec![1, 20, 4, 10]) == false);
        assert!(monotonic(vec![4, 1, 0, -10]) == true);
        assert!(monotonic(vec![4, 1, 1, 0]) == true);
        assert!(monotonic(vec![1, 2, 3, 2, 5, 60]) == false);
        assert!(monotonic(vec![1, 2, 3, 4, 5, 60]) == true);
        assert!(monotonic(vec![9, 9, 9, 9]) == true);
    }

}
