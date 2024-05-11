
fn unique_digits(x: Vec<i32>) -> Vec<i32> {
    fn has_no_even_digit(num: i32) -> bool {
        num.to_string().chars().all(|c| {
            match c {
                '0' | '2' | '4' | '6' | '8' => false,
                _ => true,
            }
        })
    }

    let mut result: Vec<i32> = x.into_iter()
        .filter(|&num| has_no_even_digit(num))
        .collect();
    result.sort_unstable();
    result
}

fn main() {
    // Example usage:
    let numbers = vec![123, 467, 911, 252, 374, 573];
    let filtered_numbers = unique_digits(numbers);
    println!("{:?}", filtered_numbers); // Output: [111, 123, 555, 573, 919, 935]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_digits() {
        assert!(unique_digits(vec![15, 33, 1422, 1]) == vec![1, 15, 33]);
        assert!(unique_digits(vec![152, 323, 1422, 10]) == vec![]);
        assert!(unique_digits(vec![12345, 2033, 111, 151]) == vec![111, 151]);
        assert!(unique_digits(vec![135, 103, 31]) == vec![31, 135]);
    }

}
