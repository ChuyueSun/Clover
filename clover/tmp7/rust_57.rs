
fn monotonic(l: Vec<i32>) -> bool {
    if l.len() < 2 {
        return true;
    }

    let mut increasing = true;
    let mut decreasing = true;

    for i in 1..l.len() {
        if l[i] < l[i - 1] {
            increasing = false;
        }
        if l[i] > l[i - 1] {
            decreasing = false;
        }
    }

    increasing || decreasing
}

fn main() {
    // Example usage:
    println!("{}", monotonic(vec![1, 2, 3, 4, 5])); // Should return true
    println!("{}", monotonic(vec![5, 4, 3, 2, 1])); // Should return true
    println!("{}", monotonic(vec![1, 3, 2])); // Should return false
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
