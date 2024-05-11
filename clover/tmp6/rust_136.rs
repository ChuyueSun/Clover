
fn largest_smallest_integers(lst: Vec<i32>) -> (Option<i32>, Option<i32>) {
    let mut largest_neg: Option<i32> = None;
    let mut smallest_pos: Option<i32> = None;

    for &num in &lst {
        if num < 0 {
            largest_neg = Some(match largest_neg {
                Some(n) if num > n => num,
                None => num,
                _ => largest_neg.unwrap(),
            });
        } else if num > 0 {
            smallest_pos = Some(match smallest_pos {
                Some(n) if num < n => num,
                None => num,
                _ => smallest_pos.unwrap(),
            });
        }
    }

    (largest_neg, smallest_pos)
}

fn main() {
    // Example usage:
    let numbers = vec![1, -5, 3, 0, -2, 4, -1];
    let (largest_negative, smallest_positive) = largest_smallest_integers(numbers);
    println!("Largest negative integer: {:?}", largest_negative);
    println!("Smallest positive integer: {:?}", smallest_positive);
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
