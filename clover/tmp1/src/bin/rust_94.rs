
// External crate to leverage existing functionality for prime checking
extern crate primal;

fn skjkasdkd(lst: Vec<i32>) -> i32 {
    lst.into_iter()
        // Filter out non-prime and negative numbers
        .filter(|&x| x > 1 && primal::is_prime(x as u64))
        // Find the maximum prime value
        .max()
        // If a maximum prime value is found, sum its digits
        .map_or(0, |max_prime| {
            max_prime.to_string().chars().map(|d| d.to_digit(10).unwrap() as i32).sum()
        })
}

fn main() {
    // Example usage:
    let nums = vec![12, 17, 4, 7, 23, 13, 42];
    println!("The sum of the digits of the largest prime is {}", skjkasdkd(nums));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skjkasdkd() {
        assert!(
            skjkasdkd(vec![
                0, 3, 2, 1, 3, 5, 7, 4, 5, 5, 5, 2, 181, 32, 4, 32, 3, 2, 32, 324, 4, 3
            ]) == 10
        );
        assert!(
            skjkasdkd(vec![
                1, 0, 1, 8, 2, 4597, 2, 1, 3, 40, 1, 2, 1, 2, 4, 2, 5, 1
            ]) == 25
        );
        assert!(
            skjkasdkd(vec![
                1, 3, 1, 32, 5107, 34, 83278, 109, 163, 23, 2323, 32, 30, 1, 9, 3
            ]) == 13
        );
        assert!(skjkasdkd(vec![0, 724, 32, 71, 99, 32, 6, 0, 5, 91, 83, 0, 5, 6]) == 11);
        assert!(skjkasdkd(vec![0, 81, 12, 3, 1, 21]) == 3);
        assert!(skjkasdkd(vec![0, 8, 1, 2, 1, 7]) == 7);
        assert!(skjkasdkd(vec![8191]) == 19);
        assert!(skjkasdkd(vec![8191, 123456, 127, 7]) == 19);
        assert!(skjkasdkd(vec![127, 97, 8192]) == 10);
    }


}
