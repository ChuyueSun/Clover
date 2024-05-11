
fn unique_digits(x: Vec<i32>) -> Vec<i32> {
    x.into_iter()
        .filter(|&num| !num.to_string().chars().any(|c| c == '0' || c == '2' || c == '4' || c == '6' || c == '8'))
        .collect::<Vec<i32>>()
}

fn main() {
    let numbers = vec![123, 234, 567, 890, 321, 345];
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
