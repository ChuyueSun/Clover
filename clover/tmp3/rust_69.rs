
use std::collections::HashMap;

fn search(lst: Vec<i32>) -> i32 {
    let mut frequency_map = HashMap::new();

    for &number in lst.iter() {
        *frequency_map.entry(number).or_insert(0) += 1;
    }

    let mut result = -1;
    for (&number, &frequency) in frequency_map.iter() {
        if number > 0 && frequency >= number && number > result {
            result = number;
        }
    }

    result
}

fn main() {
    // Example use:
    let lst = vec![1, 2, 2, 3, 3, 3];
    println!("The result is: {}", search(lst));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        assert!(search(vec![5, 5, 5, 5, 1]) == 1);
        assert!(search(vec![4, 1, 4, 1, 4, 4]) == 4);
        assert!(search(vec![3, 3]) == -1);
        assert!(search(vec![8, 8, 8, 8, 8, 8, 8, 8]) == 8);
        assert!(search(vec![2, 3, 3, 2, 2]) == 2);
        assert!(
            search(vec![
                2, 7, 8, 8, 4, 8, 7, 3, 9, 6, 5, 10, 4, 3, 6, 7, 1, 7, 4, 10, 8, 1
            ]) == 1
        );
        assert!(search(vec![3, 2, 8, 2]) == 2);
        assert!(search(vec![6, 7, 1, 8, 8, 10, 5, 8, 5, 3, 10]) == 1);
        assert!(search(vec![8, 8, 3, 6, 5, 6, 4]) == -1);
        assert!(
            search(vec![
                6, 9, 6, 7, 1, 4, 7, 1, 8, 8, 9, 8, 10, 10, 8, 4, 10, 4, 10, 1, 2, 9, 5, 7, 9
            ]) == 1
        );
        assert!(search(vec![1, 9, 10, 1, 3]) == 1);
        assert!(
            search(vec![
                6, 9, 7, 5, 8, 7, 5, 3, 7, 5, 10, 10, 3, 6, 10, 2, 8, 6, 5, 4, 9, 5, 3, 10
            ]) == 5
        );
        assert!(search(vec![1]) == 1);
        assert!(
            search(vec![
                8, 8, 10, 6, 4, 3, 5, 8, 2, 4, 2, 8, 4, 6, 10, 4, 2, 1, 10, 2, 1, 1, 5
            ]) == 4
        );
        assert!(
            search(vec![
                2, 10, 4, 8, 2, 10, 5, 1, 2, 9, 5, 5, 6, 3, 8, 6, 4, 10
            ]) == 2
        );
        assert!(search(vec![1, 6, 10, 1, 6, 9, 10, 8, 6, 8, 7, 3]) == 1);
        assert!(
            search(vec![
                9, 2, 4, 1, 5, 1, 5, 2, 5, 7, 7, 7, 3, 10, 1, 5, 4, 2, 8, 4, 1, 9, 10, 7, 10, 2, 8,
                10, 9, 4
            ]) == 4
        );
        assert!(
            search(vec![
                2, 6, 4, 2, 8, 7, 5, 6, 4, 10, 4, 6, 3, 7, 8, 8, 3, 1, 4, 2, 2, 10, 7
            ]) == 4
        );
        assert!(
            search(vec![
                9, 8, 6, 10, 2, 6, 10, 2, 7, 8, 10, 3, 8, 2, 6, 2, 3, 1
            ]) == 2
        );
        assert!(
            search(vec![
                5, 5, 3, 9, 5, 6, 3, 2, 8, 5, 6, 10, 10, 6, 8, 4, 10, 7, 7, 10, 8
            ]) == -1
        );
        assert!(search(vec![10]) == -1);
        assert!(search(vec![9, 7, 7, 2, 4, 7, 2, 10, 9, 7, 5, 7, 2]) == 2);
        assert!(search(vec![5, 4, 10, 2, 1, 1, 10, 3, 6, 1, 8]) == 1);
        assert!(
            search(vec![
                7, 9, 9, 9, 3, 4, 1, 5, 9, 1, 2, 1, 1, 10, 7, 5, 6, 7, 6, 7, 7, 6
            ]) == 1
        );
        assert!(search(vec![3, 10, 10, 9, 2]) == -1);
    }

}