
fn sort_numbers(numbers: String) -> String {
    let number_dict = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut num_vec: Vec<_> = numbers
        .split_whitespace()
        .filter_map(|word| number_dict.iter().position(|&n| n == word))
        .collect();
    num_vec.sort_unstable();

    num_vec
        .into_iter()
        .map(|index| number_dict[index])
        .collect::<Vec<&str>>()
        .join(" ")
}

fn main() {
    // Example usage:
    let input = "three one four".to_string();
    let sorted = sort_numbers(input);
    println!("{}", sorted); // Should print "one three four"
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_sort_numbers() {
        assert!(sort_numbers("".to_string()) == "".to_string());
        assert!(sort_numbers("three".to_string()) == "three".to_string());
        assert!(sort_numbers("three five nine".to_string()) == "three five nine");
        assert!(
            sort_numbers("five zero four seven nine eight".to_string())
                == "zero four five seven eight nine".to_string()
        );
        assert!(
            sort_numbers("six five four three two one zero".to_string())
                == "zero one two three four five six".to_string()
        );
    }

}
