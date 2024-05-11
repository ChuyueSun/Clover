
fn pluck(arr: Vec<i32>) -> Vec<i32> {
    let mut smallest_even = None;
    let mut smallest_index = None;

    for (index, &value) in arr.iter().enumerate() {
        if value % 2 == 0 {
            match smallest_even {
                None => {
                    smallest_even = Some(value);
                    smallest_index = Some(index as i32);
                }
                Some(current_smallest) if value < current_smallest => {
                    smallest_even = Some(value);
                    smallest_index = Some(index as i32);
                }
                _ => {}
            }
        }
    }

    match smallest_even {
        Some(value) => vec![value, smallest_index.unwrap()],
        None => vec![],
    }
}

fn main() {
    // Example usage:
    let branch = vec![3, 5, 2, 9, 4, 6];
    println!("{:?}", pluck(branch)); // Should print: [2, 2]
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
