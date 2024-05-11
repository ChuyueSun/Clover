
fn sort_array_1(mut arr: Vec<i32>) -> Vec<i32> {
    use std::cmp::Ordering;

    // Closure to count number of ones in binary representation
    let count_ones = |mut x: i32| -> i32 {
        let mut count = 0;
        while x > 0 {
            count += x & 1;
            x >>= 1;
        }
        count
    };

    arr.sort_by(|&a, &b| {
        let a_ones = count_ones(a);
        let b_ones = count_ones(b);

        // Compare based on number of ones first, then by value
        match a_ones.cmp(&b_ones) {
            Ordering::Equal => a.cmp(&b),
            other => other,
        }
    });

    arr
}

// Example:
// let arr = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
// let sorted = sort_array_1(arr);
// println!("{:?}", sorted); // Output: [0, 1, 2, 4, 8, 3, 5, 6, 9, 10, 7]

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_array_1() {
        assert!(sort_array_1(vec![1, 5, 2, 3, 4]) == vec![1, 2, 4, 3, 5]);
        assert!(sort_array_1(vec![-2, -3, -4, -5, -6]) == vec![-4, -2, -6, -5, -3]);
        assert!(sort_array_1(vec![1, 0, 2, 3, 4]) == vec![0, 1, 2, 4, 3]);
        assert!(sort_array_1(vec![]) == vec![]);
        assert!(
            sort_array_1(vec![2, 5, 77, 4, 5, 3, 5, 7, 2, 3, 4])
                == vec![2, 2, 4, 4, 3, 3, 5, 5, 5, 7, 77]
        );
        assert!(sort_array_1(vec![3, 6, 44, 12, 32, 5]) == vec![32, 3, 5, 6, 12, 44]);
        assert!(sort_array_1(vec![2, 4, 8, 16, 32]) == vec![2, 4, 8, 16, 32]);
        assert!(sort_array_1(vec![2, 4, 8, 16, 32]) == vec![2, 4, 8, 16, 32]);
    }

}
