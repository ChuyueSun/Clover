
fn by_length(arr: Vec<i32>) -> Vec<String> {
    let digit_names = [
        "", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine",
    ];

    let mut filtered_sorted: Vec<i32> = arr
        .into_iter()
        .filter(|&x| x >= 1 && x <= 9)
        .collect();
    
    filtered_sorted.sort_unstable();
    filtered_sorted.reverse();

    filtered_sorted
        .into_iter()
        .map(|x| digit_names[x as usize].to_string())
        .collect()
}

fn main() {
    let arr = vec![3, 13, 7, 2, 9, 10];
    let result = by_length(arr);
    println!("{:?}", result);
}
`
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
