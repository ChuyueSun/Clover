
fn by_length(mut arr: Vec<i32>) -> Vec<String> {
    // Define a hashmap to map digits to their corresponding names.
    let names = vec![
        "",
        "One",
        "Two",
        "Three",
        "Four",
        "Five",
        "Six",
        "Seven",
        "Eight",
        "Nine",
    ];
    
    // Retain only the integers that are between 1 and 9.
    arr.retain(|&x| (1..=9).contains(&x));

    // Sort the retained integers
    arr.sort_unstable();

    // Reverse the sorted array.
    arr.reverse();

    // Replace each digit by its corresponding name
    arr.into_iter()
        .map(|x| names[x as usize].to_string())
        .collect()
}

// Example usage
fn main() {
    let numbers = vec![5, 1, 5, 8, 12, 3, 7];
    let result = by_length(numbers);
    println!("{:?}", result); // Should print: ["Nine", "Eight", "Seven", "Five", "Five", "Three", "One"]
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
