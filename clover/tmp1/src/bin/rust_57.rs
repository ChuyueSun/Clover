
fn monotonic(l: Vec<i32>) -> bool {
    if l.is_empty() || l.len() == 1 {
        return true;
    }

    let mut inc = true;
    let mut dec = true;

    for i in 1..l.len() {
        if l[i] > l[i - 1] {
            dec = false;
        } else if l[i] < l[i - 1] {
            inc = false;
        }
    }

    inc || dec
}

fn main() {
    // Example usage:
    // These should return true
    println!("{}", monotonic(vec![1, 2, 3, 4, 5]));
    println!("{}", monotonic(vec![5, 4, 3, 2, 1]));
    // This should return false
    println!("{}", monotonic(vec![1, 3, 2]));
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
