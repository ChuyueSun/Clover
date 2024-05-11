
fn sum_squares_cubes(lst: Vec<i32>) -> i32 {
    lst.iter()
        .enumerate()
        .map(|(index, &value)| {
            if index % 3 == 0 {
                value.pow(2)
            } else if index % 4 == 0 && index % 3 != 0 {
                value.pow(3)
            } else {
                value
            }
        })
        .sum()
}

fn main() {
    // Example usage:
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("The sum is: {}", sum_squares_cubes(numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_squares() {
        assert_eq!(sum_squares(vec![1.0, 2.0, 3.0]), 14);
        assert_eq!(sum_squares(vec![1.0, 2.0, 3.0]), 14);
        assert_eq!(sum_squares(vec![1.0, 3.0, 5.0, 7.0]), 84);
        assert_eq!(sum_squares(vec![1.4, 4.2, 0.0]), 29);
        assert_eq!(sum_squares(vec![-2.4, 1.0, 1.0]), 6);
        assert_eq!(sum_squares(vec![100.0, 1.0, 15.0, 2.0]), 10230);
        assert_eq!(sum_squares(vec![10000.0, 10000.0]), 200000000);
        assert_eq!(sum_squares(vec![-1.4, 4.6, 6.3]), 75);
        assert_eq!(sum_squares(vec![-1.4, 17.9, 18.9, 19.9]), 1086);
        assert_eq!(sum_squares(vec![0.0]), 0);
        assert_eq!(sum_squares(vec![-1.0]), 1);
        assert_eq!(sum_squares(vec![-1.0, 1.0, 0.0]), 2);
    }

}
