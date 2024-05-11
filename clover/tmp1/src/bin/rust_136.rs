
fn largest_smallest_integers(lst: Vec<i32>) -> (Option<i32>, Option<i32>) {
    let mut largest_negative = None;
    let mut smallest_positive = None;

    for &item in lst.iter() {
        if item < 0 {
            largest_negative = Some(match largest_negative {
                None => item,
                Some(current_largest) => current_largest.max(item),
            });
        } else if item > 0 {
            smallest_positive = Some(match smallest_positive {
                None => item,
                Some(current_smallest) => current_smallest.min(item),
            });
        }
    }

    (largest_negative, smallest_positive)
}

fn main() {
    // Example usage:
    let numbers = vec![-5, -3, -1, 0, 2, 4, 6];
    let (largest_negative, smallest_positive) = largest_smallest_integers(numbers);
    println!("{:?}, {:?}", largest_negative, smallest_positive); // Output: Some(-1), Some(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_smallest_integers() {
        assert_eq!(
            largest_smallest_integers(vec![2, 4, 1, 3, 5, 7]),
            vec![0, 1]
        );
        assert_eq!(
            largest_smallest_integers(vec![2, 4, 1, 3, 5, 7, 0]),
            vec![0, 1]
        );
        assert_eq!(
            largest_smallest_integers(vec![1, 3, 2, 4, 5, 6, -2]),
            vec![-2, 1]
        );
        assert_eq!(
            largest_smallest_integers(vec![4, 5, 3, 6, 2, 7, -7]),
            vec![-7, 2]
        );
        assert_eq!(
            largest_smallest_integers(vec![7, 3, 8, 4, 9, 2, 5, -9]),
            vec![-9, 2]
        );
        assert_eq!(largest_smallest_integers(vec![]), vec![0, 0]);
        assert_eq!(largest_smallest_integers(vec![0]), vec![0, 0]);
        assert_eq!(largest_smallest_integers(vec![-1, -3, -5, -6]), vec![-1, 0]);
        assert_eq!(
            largest_smallest_integers(vec![-1, -3, -5, -6, 0]),
            vec![-1, 0]
        );
        assert_eq!(
            largest_smallest_integers(vec![-6, -4, -4, -3, 1]),
            vec![-3, 1]
        );
        assert_eq!(
            largest_smallest_integers(vec![-6, -4, -4, -3, -100, 1]),
            vec![-3, 1]
        );
    }

}
