
fn order_by_digits_sum(arr: Vec<i32>) -> Vec<i32> {
    let mut indexed_sums: Vec<_> = arr.iter().enumerate().map(|(index, &value)| {
        let sum_of_digits = value.to_string().chars()
            .map(|d| d.to_digit(10).unwrap_or(0) as i32)
            .sum::<i32>();
        (sum_of_digits, index, value)
    }).collect();

    indexed_sums.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

    indexed_sums.into_iter().map(|(_, _, value)| value).collect()
}

fn main() {
    let arr = vec![34, 23, 12, 45, 9, 1, 101];
    let ordered = order_by_digits_sum(arr);
    println!("{:?}", ordered);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_by_points() {
        assert_eq!(
            order_by_points(vec![1, 11, -1, -11, -12]),
            vec![-1, -11, 1, -12, 11]
        );
        assert_eq!(
            order_by_points(vec![
                1234, 423, 463, 145, 2, 423, 423, 53, 6, 37, 3457, 3, 56, 0, 46
            ]),
            vec![0, 2, 3, 6, 53, 423, 423, 423, 1234, 145, 37, 46, 56, 463, 3457]
        );
        assert_eq!(order_by_points(vec![]), vec![]);
        assert_eq!(
            order_by_points(vec![1, -11, -32, 43, 54, -98, 2, -3]),
            vec![-3, -32, -98, -11, 1, 2, 43, 54]
        );
        assert_eq!(
            order_by_points(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]),
            vec![1, 10, 2, 11, 3, 4, 5, 6, 7, 8, 9]
        );
        assert_eq!(
            order_by_points(vec![0, 6, 6, -76, -21, 23, 4]),
            vec![-76, -21, 0, 4, 23, 6, 6]
        );
    }

}
