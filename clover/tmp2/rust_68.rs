
fn pluck(arr: Vec<i32>) -> Vec<i32> {
    let mut smallest_even = None;
    let mut index = 0;

    for (idx, &value) in arr.iter().enumerate() {
        if value % 2 == 0 {
            if smallest_even.is_none() || value < smallest_even.unwrap() {
                smallest_even = Some(value);
                index = idx;
            }
        }
    }

    smallest_even.map_or_else(Vec::new, |v| vec![v, index as i32])
}

fn main() {
    // Example test cases
    println!("{:?}", pluck(vec![1, 4, 3, 2])); // should print [2, 3]
    println!("{:?}", pluck(vec![7, 3, 9])); // should print []
    println!("{:?}", pluck(vec![])); // should print []
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
