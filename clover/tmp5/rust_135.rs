
fn can_arrange(arr: Vec<i32>) -> i32 {
    for (i, window) in arr.windows(2).enumerate().rev() {
        if window[0] > window[1] {
            return i as i32 + 1; // i is the index of the first element, so +1 gives the index of the second
        }
    }
    -1
}

fn main() {
    // Example usage
    let arr = vec![1, 2, 5, 3, 4];
    println!("{}", can_arrange(arr)); // Should print 3 since element at index 3 is 3 and element at index 2 is 5
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
