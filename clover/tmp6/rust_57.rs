
fn monotonic(l: Vec<i32>) -> bool {
    if l.len() <= 1 {
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
    // Example usage:
    let v1 = vec![1, 2, 3, 4, 5];
    println!("Is the list monotonic: {}", monotonic(v1)); 

    let v2 = vec![5, 4, 3, 2, 1];
    println!("Is the list monotonic: {}", monotonic(v2));

    let v3 = vec![1, 3, 2];
    println!("Is the list monotonic: {}", monotonic(v3));
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
