
fn can_arrange(arr: Vec<i32>) -> i32 {
    for (i, window) in arr.windows(2).enumerate() {
        if window[1] < window[0] {
            return i as i32 + 1;  // Plus one because enumerate starts from zero
        }
    }
    -1
}

fn main() {
    // Example usage:
    let arr = vec![1, 2, 5, 3, 6];
    println!("{}", can_arrange(arr)); // should print 3, which is the index of 3 that follows 5
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
