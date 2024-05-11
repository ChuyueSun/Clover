
fn count_nums(n: Vec<i32>) -> usize {
    n.iter()
        .filter(|&&x| {
            x.to_string()
                .chars()
                .filter_map(|c| c.to_digit(10))
                .sum::<u32>() > 0
        })
        .count()
}

fn main() {
    let numbers = vec![15, -23, 0, 5, -9];
    println!("The count is: {}", count_nums(numbers));
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
