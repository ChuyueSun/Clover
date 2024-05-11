
fn by_length(arr: Vec<i32>) -> Vec<String> {
    let mut filtered_sorted_reversed: Vec<_> = arr
        .into_iter()
        .filter(|&n| (1..=9).contains(&n))
        .collect();
        
    filtered_sorted_reversed.sort_unstable();
    filtered_sorted_reversed.reverse();
    
    filtered_sorted_reversed
        .into_iter()
        .map(|num| match num {
            1 => "One".to_string(),
            2 => "Two".to_string(),
            3 => "Three".to_string(),
            4 => "Four".to_string(),
            5 => "Five".to_string(),
            6 => "Six".to_string(),
            7 => "Seven".to_string(),
            8 => "Eight".to_string(),
            9 => "Nine".to_string(),
            _ => unreachable!(), // As per filtered range 1..=9
        })
        .collect()
}

fn main() {
    // Example usage
    let input = vec![4, 1, 7, 3, 11, 9];
    let output = by_length(input);
    println!("{:?}", output); // Output should be: ["Nine", "Seven", "Four", "Three", "One"]
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
