
fn pluck(arr: Vec<i32>) -> Vec<i32> {
    let mut smallest_even = None;
    
    for (index, &value) in arr.iter().enumerate() {
        if value % 2 == 0 {
            if let Some((_, smallest)) = smallest_even {
                if value < smallest {
                    smallest_even = Some((index, value));
                }
            } else {
                smallest_even = Some((index, value));
            }
        }
    }
    
    smallest_even.map(|(index, value)| vec![value, index as i32]).unwrap_or_else(Vec::new)
}

// Example usage (uncomment for testing)
// fn main() {
//     let branch = vec![5, 3, 2, 4, 9, 7, 8];
//     println!("{:?}", pluck(branch));
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pluck() {
        assert!(pluck(vec![4, 2, 3]) == vec![2, 1]);
        assert!(pluck(vec![1, 2, 3]) == vec![2, 1]);
        assert!(pluck(vec![]) == vec![]);
        assert!(pluck(vec![5, 0, 3, 0, 4, 2]) == vec![0, 1]);
        assert!(pluck(vec![1, 2, 3, 0, 5, 3]) == vec![0, 3]);
        assert!(pluck(vec![5, 4, 8, 4, 8]) == vec![4, 1]);
        assert!(pluck(vec![7, 6, 7, 1]) == vec![6, 1]);
        assert!(pluck(vec![7, 9, 7, 1]) == vec![]);
    }

}
