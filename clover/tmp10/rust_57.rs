
fn monotonic(l: Vec<i32>) -> bool {
    if l.is_empty() || l.len() == 1 {
        return true;
    }
    let mut increasing = true;
    let mut decreasing = true;

    for i in 1..l.len() {
        if l[i] > l[i - 1] {
            decreasing = false;
        } else if l[i] < l[i - 1] {
            increasing = false;
        }
        if !increasing && !decreasing {
            return false;
        }
    }

    increasing || decreasing
}

fn main() {
    // Example usage:
    let l1 = vec![1, 2, 3, 4, 5];
    let l2 = vec![5, 4, 3, 2, 1];
    let l3 = vec![1, 2, 2, 3, 4];
    let l4 = vec![1, 3, 2];

    println!("l1 is monotonic: {}", monotonic(l1)); // true
    println!("l2 is monotonic: {}", monotonic(l2)); // true
    println!("l3 is monotonic: {}", monotonic(l3)); // true
    println!("l4 is monotonic: {}", monotonic(l4)); // false
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
