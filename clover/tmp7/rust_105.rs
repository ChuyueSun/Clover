
fn by_length(mut arr: Vec<i32>) -> Vec<String> {
    let digit_to_word = |digit: &i32| -> String {
        match digit {
            1 => "One",
            2 => "Two",
            3 => "Three",
            4 => "Four",
            5 => "Five",
            6 => "Six",
            7 => "Seven",
            8 => "Eight",
            9 => "Nine",
            _ => "",
        }
        .to_string()
    };
    
    arr.retain(|&x| (1..=9).contains(&x));
    arr.sort_unstable();
    arr.reverse();
    arr.into_iter().map(digit_to_word).collect()
}

fn main() {
    // Example usage:
    let input = vec![0, 3, 5, 2, 1, 20, 9];
    let result = by_length(input);
    println!("{:?}", result); // Should print: ["Nine", "Five", "Three", "Two", "One"]
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
