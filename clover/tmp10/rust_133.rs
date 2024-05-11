
fn sum_squares(lst: Vec<f32>) -> f32 {
    lst.iter()
        .enumerate()
        .map(|(index, &val)| {
            if index % 3 == 0 {
                val.powi(2)
            } else if index % 4 == 0 {
                val.powi(3)
            } else {
                val
            }
        })
        .sum()
}

fn main() {
    let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    println!("The sum is: {}", sum_squares(numbers));
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
