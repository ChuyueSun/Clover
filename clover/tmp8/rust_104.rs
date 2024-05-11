
fn unique_digits(mut x: Vec<i32>) -> Vec<i32> {
    x.retain(|&num| num.to_string().chars().all(|c| c != '0' && c != '2' && c != '4' && c != '6' && c != '8'));
    x.sort_unstable();
    x
}

fn main() {
    let nums = vec![123, 456, 789, 113, 407];
    let result = unique_digits(nums);
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
