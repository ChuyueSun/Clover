
fn by_length(arr: Vec<i32>) -> Vec<String> {
    // Declare a vector to hold the sorted digits
    let mut sorted_digits: Vec<i32> = arr.into_iter()
        .filter(|&x| x >= 1 && x <= 9)
        .collect();
    
    // Sort and reverse the vector of valid digits
    sorted_digits.sort();
    sorted_digits.reverse();
    
    // Replace each digit with its corresponding name
    let digit_names = ["One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine"];
    let result: Vec<String> = sorted_digits
        .into_iter()
        .map(|x| digit_names[(x - 1) as usize].to_string())
        .collect();

    result
}

fn main() {
    // Test the function by_length()
    let test_array = vec![3, 1, 4, 7, 8, 9, 10, 0, 2, 6, 5];
    let sorted_array = by_length(test_array);
    for name in sorted_array {
        println!("{}", name);
    }
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
