
fn by_length(arr: Vec<i32>) -> Vec<String> {
    let mut sorted: Vec<_> = arr.into_iter()
        .filter(|&x| x >= 1 && x <= 9)
        .collect();
    sorted.sort_unstable();
    sorted.reverse();

    sorted.iter().map(|&digit| match digit {
        1 => "One".to_string(),
        2 => "Two".to_string(),
        3 => "Three".to_string(),
        4 => "Four".to_string(),
        5 => "Five".to_string(),
        6 => "Six".to_string(),
        7 => "Seven".to_string(),
        8 => "Eight".to_string(),
        9 => "Nine".to_string(),
        _ => unreachable!(),
    }).collect()
}

fn main() {
    let arr = vec![7, 13, 9, 2, 5, 1, 10];
    let result = by_length(arr);
    println!("{:?}", result);
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
