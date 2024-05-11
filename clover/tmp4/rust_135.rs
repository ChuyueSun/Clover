
fn can_arrange(arr: Vec<i32>) -> i32 {
    for i in (1..arr.len()).rev() {
        if arr[i] < arr[i - 1] {
            return i as i32;
        }
    }
    -1
}

fn main() {
    // Example usage:
    let numbers = vec![1, 2, 4, 3, 5];
    println!("{}", can_arrange(numbers)); // Output should be 3 (index of element `3`)
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_can_arrange() {
        assert_eq!(can_arrange(vec![1, 2, 4, 3, 5]), 3);
        assert_eq!(can_arrange(vec![1, 2, 4, 5]), -1);
        assert_eq!(can_arrange(vec![1, 4, 2, 5, 6, 7, 8, 9, 10]), 2);
        assert_eq!(can_arrange(vec![4, 8, 5, 7, 3]), 4);
        assert_eq!(can_arrange(vec![]), -1);
    }


}
