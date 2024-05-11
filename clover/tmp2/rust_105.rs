
fn by_length(arr: Vec<i32>) -> Vec<String> {
    let mut filtered_and_sorted: Vec<i32> = arr
        .into_iter()
        .filter(|&x| (1..=9).contains(&x))
        .collect();
    filtered_and_sorted.sort_unstable();
    filtered_and_sorted.reverse();
    // Replace each digit with its corresponding name
    filtered_and_sorted
        .into_iter()
        .map(|x| match x {
            1 => "One".to_string(),
            2 => "Two".to_string(),
            3 => "Three".to_string(),
            4 => "Four".to_string(),
            5 => "Five".to_string(),
            6 => "Six".to_string(),
            7 => "Seven".to_string(),
            8 => "Eight".to_string(),
            9 => "Nine".to_string(),
            _ => unreachable!(), // This case cannot happen due to the filter applied before
        })
        .collect()
}

fn main() {
    let numbers = vec![1, 3, 5, 7, 9, 4, 15, 0];
    let sorted_names = by_length(numbers);
    println!("{:?}", sorted_names);
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
