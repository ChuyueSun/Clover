
fn monotonic(l: Vec<i32>) -> bool {
    if l.is_empty() {
        return true;
    }

    let mut growing = false;
    let mut shrinking = false;
    
    for i in 0..l.len() - 1 {
        if l[i] < l[i + 1] {
            growing = true;
        } else if l[i] > l[i + 1] {
            shrinking = true;
        }

        if growing && shrinking {
            return false;
        }
    }

    true
}

fn main() {
    // Example usage:
    let nums1 = vec![1, 2, 2, 3];
    let nums2 = vec![3, 2, 1];
    let nums3 = vec![1, 3, 2];

    println!("nums1 is monotonic: {}", monotonic(nums1)); // true
    println!("nums2 is monotonic: {}", monotonic(nums2)); // true
    println!("nums3 is monotonic: {}", monotonic(nums3)); // false
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
