
/// Calculate the first `n + 1` numbers of the Tribonacci sequence,
/// where `tri(1)` is defined as `3`, `tri(n)` for even `n` is `1 + n / 2`,
/// and for odd `n` it's defined recursively using the previous, next and 
/// double-next values in the sequence.
///
/// # Arguments
///
/// * `n`: A non-negative `i32` integer for which the first `n + 1` Tribonacci numbers will be calculated.
///
/// # Returns
///
/// A `Vec<i32>` containing the first `n + 1` numbers of the Tribonacci sequence.
fn tri(n: i32) -> Vec<i32> {
    let mut tribonacci = vec![0; n as usize + 1];

    for i in 1..=n as usize {
        if i == 1 {
            tribonacci[i] = 3;
        } else if i % 2 == 0 {
            tribonacci[i] = 1 + i as i32 / 2;
        } else {
            tribonacci[i] = if i+1 <= n as usize {
                tribonacci[i - 1] + tribonacci[i - 2] + tribonacci[i + 1]
            } else {
                tribonacci[i - 1] + tribonacci[i - 2]
            };
        }
    }

    tribonacci
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tri() {
        assert!(tri(3) == vec![1, 3, 2, 8]);
        assert!(tri(4) == vec![1, 3, 2, 8, 3]);
        assert!(tri(5) == vec![1, 3, 2, 8, 3, 15]);
        assert!(tri(6) == vec![1, 3, 2, 8, 3, 15, 4]);
        assert!(tri(7) == vec![1, 3, 2, 8, 3, 15, 4, 24]);
        assert!(tri(8) == vec![1, 3, 2, 8, 3, 15, 4, 24, 5]);
        assert!(tri(9) == vec![1, 3, 2, 8, 3, 15, 4, 24, 5, 35]);
        assert!(
            tri(20)
                == vec![1, 3, 2, 8, 3, 15, 4, 24, 5, 35, 6, 48, 7, 63, 8, 80, 9, 99, 10, 120, 11]
        );
        assert!(tri(0) == vec![1]);
        assert!(tri(1) == vec![1, 3]);
    }

}
