
fn smallest_change(arr: Vec<i32>) -> i32 {
    let mut changes = 0;
    let n = arr.len();
    let mut left = 0;
    let mut right = n - 1;

    while left < right {
        if arr[left] != arr[right] {
            changes += 1;
        }
        left += 1;
        right -= 1;
    }
    
    changes
}

fn main() {
    let arr = vec![1, 2, 3, 4, 3, 2, 1]; // Example array
    println!("Minimum changes to make array palindromic: {}", smallest_change(arr));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_change() {
        assert!(smallest_change(vec![1, 2, 3, 5, 4, 7, 9, 6]) == 4);
        assert!(smallest_change(vec![1, 2, 3, 4, 3, 2, 2]) == 1);
        assert!(smallest_change(vec![1, 4, 2]) == 1);
        assert!(smallest_change(vec![1, 4, 4, 2]) == 1);
        assert!(smallest_change(vec![1, 2, 3, 2, 1]) == 0);
        assert!(smallest_change(vec![3, 1, 1, 3]) == 0);
        assert!(smallest_change(vec![1]) == 0);
        assert!(smallest_change(vec![0, 1]) == 1);
    }

}
