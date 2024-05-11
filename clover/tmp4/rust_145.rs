// Function to sort the given list according to the sum of their digits
fn order_by_points(arr: Vec<i32>) -> Vec<i32> {
    // Helper function to compute the sum of digits of an integer
    fn digit_sum(num: i32) -> i32 {
        num.abs().to_string().chars().map(|c| c.to_digit(10).unwrap_or(0) as i32).sum()
    }

    // Enumerate to keep track of original indices
    let mut indexed_arr: Vec<(usize, i32)> = arr.into_iter().enumerate().collect();

    // Sort using a custom comparator
    indexed_arr.sort_by(|&(idx1, num1), &(idx2, num2)| {
        let sum1 = digit_sum(num1);
        let sum2 = digit_sum(num2);

        // Primary comparison by digit sum
        let cmp = sum1.cmp(&sum2);
        
        // If sums are the same, compare by original index
        if cmp == std::cmp::Ordering::Equal {
            idx1.cmp(&idx2)
        } else {
            cmp
        }
    });

    // Extract the sorted integers from the tuples
    indexed_arr.into_iter().map(|(_, num)| num).collect()
}

fn main() {
    let numbers = vec![56, 34, 52, 12, 19, -47];
    let sorted_numbers = order_by_points(numbers);
    println!("{:?}", sorted_numbers);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_by_points() {
        assert_eq!(
            order_by_points(vec![1, 11, -1, -11, -12]),
            vec![-1, -11, 1, -12, 11]
        );
        assert_eq!(
            order_by_points(vec![
                1234, 423, 463, 145, 2, 423, 423, 53, 6, 37, 3457, 3, 56, 0, 46
            ]),
            vec![0, 2, 3, 6, 53, 423, 423, 423, 1234, 145, 37, 46, 56, 463, 3457]
        );
        assert_eq!(order_by_points(vec![]), vec![]);
        assert_eq!(
            order_by_points(vec![1, -11, -32, 43, 54, -98, 2, -3]),
            vec![-3, -32, -98, -11, 1, 2, 43, 54]
        );
        assert_eq!(
            order_by_points(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]),
            vec![1, 10, 2, 11, 3, 4, 5, 6, 7, 8, 9]
        );
        assert_eq!(
            order_by_points(vec![0, 6, 6, -76, -21, 23, 4]),
            vec![-76, -21, 0, 4, 23, 6, 6]
        );
    }

}
