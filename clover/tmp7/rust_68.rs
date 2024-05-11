
fn pluck(arr: Vec<i32>) -> Vec<i32> {
    let mut smallest_even = None;
    for (index, &value) in arr.iter().enumerate() {
        if value % 2 == 0 {
            smallest_even = match smallest_even {
                Some((smallest, _)) if value < smallest => Some((value, index as i32)),
                None => Some((value, index as i32)),
                _ => smallest_even,
            };
        }
    }
    
    smallest_even.map_or(vec![], |(value, index)| vec![value, index])
}

fn main() {
    // Example usage:
    let nodes = vec![3, 7, 4, 1, 2];
    println!("{:?}", pluck(nodes)); // Outputs: [2, 4]
}

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
