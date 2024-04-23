fn main() {}

/*

    Write a function count_nums which takes an array of integers and returns
    the number of elements which has a sum of digits > 0.
    If a number is negative, then its first signed digit will be negative:
    e.g. -123 has signed digits -1, 2, and 3.

*/

fn count_nums(n: Vec<i32>) -> i32 {
    let mut num: i32 = 0;

    for nmbr in n {
        if nmbr > 0 {
            num += 1;
        } else {
            let mut sum: i32 = 0;
            let mut w: i32;
            w = i32::abs(nmbr);

            while w >= 10 {
                sum += w % 10;
                w = w / 10;
            }
            sum -= w;
            if sum > 0 {
                num += 1;
            }
        }
    }
    return num;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_nums() {
        assert!(count_nums(vec![]) == 0);
        assert!(count_nums(vec![-1, -2, 0]) == 0);
        assert!(count_nums(vec![1, 1, 2, -2, 3, 4, 5]) == 6);
        assert!(count_nums(vec![1, 6, 9, -6, 0, 1, 5]) == 5);
        assert!(count_nums(vec![1, 100, 98, -7, 1, -1]) == 4);
        assert!(count_nums(vec![12, 23, 34, -45, -56, 0]) == 5);
        assert!(count_nums(vec![-0, 1]) == 1);
        assert!(count_nums(vec![1]) == 1);
    }
}
