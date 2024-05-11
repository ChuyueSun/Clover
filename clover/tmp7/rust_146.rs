
fn special_filter(nums: Vec<i32>) -> usize {
    nums.iter()
        .filter(|&&num| {
            num > 10 && {
                let digits = num.to_string();
                let first_char = digits.chars().next().unwrap();
                let last_char = digits.chars().last().unwrap();
                first_char == '1'
                    || first_char == '3'
                    || first_char == '5'
                    || first_char == '7'
                    || first_char == '9'
                } && {
                    let last_char = digits.chars().last().unwrap();
                    last_char == '1'
                        || last_char == '3'
                        || last_char == '5'
                        || last_char == '7'
                        || last_char == '9'
                }
        })
        .count()
}

fn main() {
    let numbers = vec![11, 23, 36, 77, 123, 94];
    let result = special_filter(numbers);
    println!("Number of special numbers: {}", result);
}
`
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_special_filter() {
        assert_eq!(special_filter(vec![5, -2, 1, -5]), 0);
        assert_eq!(special_filter(vec![15, -73, 14, -15]), 1);
        assert_eq!(special_filter(vec![33, -2, -3, 45, 21, 109]), 2);
        assert_eq!(special_filter(vec![43, -12, 93, 125, 121, 109]), 4);
        assert_eq!(special_filter(vec![71, -2, -33, 75, 21, 19]), 3);
        assert_eq!(special_filter(vec![1]), 0);
        assert_eq!(special_filter(vec![]), 0);
    }

}
