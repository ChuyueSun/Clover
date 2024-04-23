fn main() {}

/*
You are given a list of integers.
    You need to find the largest prime value and return the sum of its digits.

*/

fn skjkasdkd(lst: Vec<i32>) -> i32 {
    let mut largest = 0;
    for i in 0..lst.len() {
        if lst[i] > largest {
            let mut prime = true;
            let mut j = 2;
            while j * j <= lst[i] {
                if lst[i] % j == 0 {
                    prime = false;
                }
                j += 1;
            }

            if prime {
                largest = lst[i];
            }
        }
    }
    let mut sum: i32 = 0;
    let mut s: String = String::new();
    s = largest.to_string();

    for n in s.chars().into_iter() {
        sum += n.to_digit(10).unwrap() as i32;
    }
    return sum;
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
