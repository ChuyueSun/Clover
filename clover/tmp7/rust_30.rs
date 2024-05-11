
fn get_positive(numbers: Vec<i32>) -> Vec<i32> {
    numbers.into_iter().filter(|&x| x > 0).collect()
}

fn main() {
    let numbers = vec![-10, -5, 0, 5, 10];
    let positive_numbers = get_positive(numbers);
    println!("{:?}", positive_numbers);
}

#[cfg(test)]
mod tests {
    use super::*;

  #[test]
    fn test_get_positive() {
        assert!(get_positive(vec![-1, -2, 4, 5, 6]) == [4, 5, 6]);
        assert!(
            get_positive(vec![5, 3, -5, 2, 3, 3, 9, 0, 123, 1, -10]) == [5, 3, 2, 3, 3, 9, 123, 1]
        );
        assert!(get_positive(vec![-1, -2]) == []);
        assert!(get_positive(vec![]) == []);
    }

}
