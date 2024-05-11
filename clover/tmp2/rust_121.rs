
fn solution(lst: Vec<i32>) -> i32 {
    lst.iter()
        .enumerate()
        .filter(|&(i, val)| i % 2 == 0 && val % 2 != 0)
        .map(|(_, val)| val)
        .sum()
}

fn main() {
    // Example usage:
    let nums = vec![1, 3, 5, 8, 6, 7]; // Values in even positions are 1, 5, 6
    println!("The sum is {}", solution(nums)); // Should output the sum of 1 and 5 (odd numbers at even positions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solutions() {
        assert_eq!(solutions(vec![5, 8, 7, 1]), 12);
        assert_eq!(solutions(vec![3, 3, 3, 3, 3]), 9);
        assert_eq!(solutions(vec![30, 13, 24, 321]), 0);
        assert_eq!(solutions(vec![5, 9]), 5);
        assert_eq!(solutions(vec![2, 4, 8]), 0);
        assert_eq!(solutions(vec![30, 13, 23, 32]), 23);
        assert_eq!(solutions(vec![3, 13, 2, 9]), 3);
    }

}
