
fn monotonic(l: Vec<i32>) -> bool {
    if l.is_empty() || l.len() == 1 {
        return true;
    }
    
    let mut decreasing = true;
    let mut increasing = true;
    
    for i in 1..l.len() {
        if l[i] > l[i - 1] {
            decreasing = false;
        }
        if l[i] < l[i - 1] {
            increasing = false;
        }
    }
    
    increasing || decreasing
}

fn main() {
    // Examples
    println!("{}", monotonic(vec![1, 2, 2, 3])); // true
    println!("{}", monotonic(vec![3, 2, 1])); // true
    println!("{}", monotonic(vec![1, 2, 0])); // false
    println!("{}", monotonic(vec![])); // true
    println!("{}", monotonic(vec![10])); // true
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
