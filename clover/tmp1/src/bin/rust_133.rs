
fn sum_squares(mut lst: Vec<f32>) -> f32 {
    for i in 0..lst.len() {
        if i % 3 == 0 {
            lst[i] = lst[i].powi(2);
        } else if i % 4 == 0 {
            lst[i] = lst[i].powi(3);
        }
    }
    lst.iter().sum()
}

fn main() {
    let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let result = sum_squares(numbers);
    println!("The result is: {}", result);
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
