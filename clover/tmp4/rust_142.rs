
fn sum_squares_142(lst: Vec<i32>) -> i32 {
    lst.iter()
        .enumerate()
        .map(|(index, &value)| {
            if index % 3 == 0 {
                value * value
            } else if index % 4 == 0 {
                value * value * value
            } else {
                value
            }
        })
        .sum()
}

fn main() {
    // Example usage:
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = sum_squares_142(numbers);
    println!("The result is: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_squares_142() {
        assert_eq!(sum_squares_142(vec![1, 2, 3]), 6);
        assert_eq!(sum_squares_142(vec![1, 4, 9]), 14);
        assert_eq!(sum_squares_142(vec![]), 0);
        assert_eq!(sum_squares_142(vec![1, 1, 1, 1, 1, 1, 1, 1, 1]), 9);
        assert_eq!(
            sum_squares_142(vec![-1, -1, -1, -1, -1, -1, -1, -1, -1]),
            -3
        );
        assert_eq!(sum_squares_142(vec![0]), 0);
        assert_eq!(sum_squares_142(vec![-1, -5, 2, -1, -5]), -126);
        assert_eq!(sum_squares_142(vec![-56, -99, 1, 0, -2]), 3030);
        assert_eq!(sum_squares_142(vec![-1, 0, 0, 0, 0, 0, 0, 0, -1]), 0);
        assert_eq!(
            sum_squares_142(vec![
                -16, -9, -2, 36, 36, 26, -20, 25, -40, 20, -4, 12, -26, 35, 37
            ]),
            -14196
        );
        assert_eq!(
            sum_squares_142(vec![
                -1, -3, 17, -1, -15, 13, -1, 14, -14, -12, -5, 14, -14, 6, 13, 11, 16, 16, 4, 10
            ]),
            -1448
        );
    }

}
