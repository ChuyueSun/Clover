
fn unique_digits(x: Vec<i32>) -> Vec<i32> {
    x.into_iter()
        .filter(|&num| {
            !num.to_string()
                .chars()
                .any(|c| match c {
                    '0' | '2' | '4' | '6' | '8' => true,
                    _ => false,
                })
        })
        .sorted()
        .collect()
}

fn main() {
    let numbers = vec![123, 234, 345, 456, 567];
    let result = unique_digits(numbers);
    println!("{:?}", result);
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
