
fn by_length(arr: Vec<i32>) -> Vec<String> {
    let mut numbers_within_range: Vec<i32> = arr.into_iter()
        .filter(|&x| (1..=9).contains(&x))
        .collect();

    numbers_within_range.sort_unstable();
    numbers_within_range.reverse();

    numbers_within_range.iter().map(|&x| match x {
        1 => "One".to_string(),
        2 => "Two".to_string(),
        3 => "Three".to_string(),
        4 => "Four".to_string(),
        5 => "Five".to_string(),
        6 => "Six".to_string(),
        7 => "Seven".to_string(),
        8 => "Eight".to_string(),
        9 => "Nine".to_string(),
        _ => unreachable!(), // Since we filtered numbers within the range 1..=9
    }).collect()
}

fn main() {
    // Example usage:
    let arr = vec![9, 3, 1, 8, 5, 2, 10, 0, -1];
    let sorted_and_named = by_length(arr);
    
    println!("{:?}", sorted_and_named); // Should display: ["Nine", "Eight", "Five", "Three", "Two", "One"]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_by_length() {
        assert!(
            by_length(vec![2, 1, 1, 4, 5, 8, 2, 3])
                == vec!["Eight", "Five", "Four", "Three", "Two", "Two", "One", "One"]
        );
        let v_empty: Vec<String> = vec![];
        assert!(by_length(vec![]) == v_empty);
        assert!(by_length(vec![1, -1, 55]) == vec!["One"]);
        assert!(by_length(vec![1, -1, 3, 2]) == vec!["Three", "Two", "One"]);
        assert!(by_length(vec![9, 4, 8]) == vec!["Nine", "Eight", "Four"]);
    }

}
