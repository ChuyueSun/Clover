
fn pluck(arr: Vec<i32>) -> Vec<i32> {
    let mut min_even_value = None;
    let mut min_index = None;

    for (index, &value) in arr.iter().enumerate() {
        if value % 2 == 0 {
            match min_even_value {
                Some(min) => {
                    if value < min {
                        min_even_value = Some(value);
                        min_index = Some(index);
                    }
                }
                None => {
                    min_even_value = Some(value);
                    min_index = Some(index);
                }
            }
        }
    }

    match min_even_value {
        Some(value) => vec![value, min_index.unwrap() as i32],
        None => vec![],
    }
}

fn main() {
    // Example usage:
    let arr = vec![3, 7, 4, 5, 2, 6];
    println!("{:?}", pluck(arr)); // Should print: [2, 4]
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
