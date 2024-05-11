
fn largest_smallest_integers(lst: Vec<i32>) -> (Option<i32>, Option<i32>) {
    let mut largest_negative = None;
    let mut smallest_positive = None;

    for &num in &lst {
        if num > 0 {
            smallest_positive = match smallest_positive {
                None => Some(num),
                Some(current_min) => Some(current_min.min(num)),
            };
        } else if num < 0 {
            largest_negative = match largest_negative {
                None => Some(num),
                Some(current_max) => Some(current_max.max(num)),
            };
        }
    }

    (largest_negative, smallest_positive)
}

fn main() {
    // Testing the function with sample vectors:
    let v1 = vec![-5, -1, -3, 0, 1, 4, 2];
    let v2 = vec![1, 2, 3, 4, 5];
    let v3 = vec![-3, -2, -1];

    println!("{:?}", largest_smallest_integers(v1));
    println!("{:?}", largest_smallest_integers(v2));
    println!("{:?}", largest_smallest_integers(v3));
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
