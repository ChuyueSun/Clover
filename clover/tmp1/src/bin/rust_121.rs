
fn solution(lst: Vec<i32>) -> i32 {
    lst.iter()
        .enumerate()
        .filter(|&(index, &value)| index % 2 == 0 && value % 2 != 0)
        .map(|(_index, &value)| value)
        .sum()
}

fn main() {
    let input = vec![1, 3, 5, 6, 7, 8]; // Example input
    println!("The sum is: {}", solution(input));
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
