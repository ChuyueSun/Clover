
fn sort_even(mut nmbs: Vec<i32>) -> Vec<i32> {
    let mut evens: Vec<i32> = nmbs.iter().enumerate()
                                   .filter(|&(i, _)| i % 2 == 0)
                                   .map(|(_, &n)| n)
                                   .collect();
    evens.sort_unstable();
    for (i, n) in nmbs.iter_mut().enumerate().filter(|&(i, _)| i % 2 == 0) {
        *n = evens.remove(0);
    }
    nmbs
}

fn main() {
    // Example usage:
    let l = vec![5, 3, 1, 2, 4];
    let l_prime = sort_even(l);
    println!("{:?}", l_prime); // Should output [1, 3, 4, 2, 5]
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_sort_even() {
        assert_eq!(sort_even(vec![1, 2, 3]), vec![1, 2, 3]);
        assert_eq!(
            sort_even(vec![5, 3, -5, 2, -3, 3, 9, 0, 123, 1, -10]),
            vec![-10, 3, -5, 2, -3, 3, 5, 0, 9, 1, 123]
        );
        assert_eq!(
            sort_even(vec![5, 8, -12, 4, 23, 2, 3, 11, 12, -10]),
            vec![-12, 8, 3, 4, 5, 2, 12, 11, 23, -10]
        );
    }

}
