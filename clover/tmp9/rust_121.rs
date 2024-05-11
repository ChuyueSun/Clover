
fn solutions(lst: Vec<i32>) -> i32 {
    lst.iter()
       .enumerate()
       .filter(|&(i, x)| i % 2 == 0 && x % 2 != 0)
       .map(|(_, x)| x)
       .sum()
}

fn main() {
    let numbers = vec![1, 3, 4, 7, 10, 11];
    println!("The sum is: {}", solutions(numbers));
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
