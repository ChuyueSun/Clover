
fn by_length(arr: Vec<i32>) -> Vec<String> {
    let mut filtered_and_sorted: Vec<i32> = arr.into_iter()
        .filter(|&x| x >= 1 && x <= 9)
        .collect();

    filtered_and_sorted.sort_unstable();
    filtered_and_sorted.reverse();

    filtered_and_sorted.into_iter().map(|x| match x {
        1 => "One".to_string(),
        2 => "Two".to_string(),
        3 => "Three".to_string(),
        4 => "Four".to_string(),
        5 => "Five".to_string(),
        6 => "Six".to_string(),
        7 => "Seven".to_string(),
        8 => "Eight".to_string(),
        9 => "Nine".to_string(),
        _ => unreachable!(), // Safe to use because we filtered to 1-9
    }).collect()
}

fn main() {
    let arr = vec![1, 2, 13, 4, 8, 3, 11, 9, 7];
    println!("{:?}", by_length(arr));
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
