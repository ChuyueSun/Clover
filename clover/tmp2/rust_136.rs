
fn largest_smallest_integers(lst: Vec<i32>) -> (Option<i32>, Option<i32>) {
    let (mut largest_negative, mut smallest_positive) = (None, None);

    for &item in lst.iter() {
        if item < 0 {
            largest_negative = match largest_negative {
                Some(ln) if item > ln => Some(item),
                None => Some(item),
                _ => largest_negative,
            };
        } else if item > 0 {
            smallest_positive = match smallest_positive {
                Some(sp) if item < sp => Some(item),
                None => Some(item),
                _ => smallest_positive,
            };
        }
    }

    (largest_negative, smallest_positive)
}

fn main() {
    let numbers = vec![-5, -7, -3, 1, 4, -2, 7, -8, 0];
    let result = largest_smallest_integers(numbers);
    println!("{:?}", result);
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
