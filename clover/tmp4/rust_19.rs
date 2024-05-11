
fn sort_numbers(numbers: String) -> String {
    // An array to represent the words for numbers 0-9
    let number_words = [
        "zero", "one", "two", "three", "four",
        "five", "six", "seven", "eight", "nine",
    ];

    // Create a vector to store the numeric values of the words
    let mut nums: Vec<_> = numbers
        .split_whitespace() // Split the input string by whitespace
        .filter_map(|word| number_words.iter().position(|&n| n == word)) // Map the words to their numeric values
        .collect(); // Collect into a vector

    // Sort the numeric values
    nums.sort_unstable();

    // Map the sorted numeric values back to words and join them into a space-separated string
    nums.iter()
        .map(|&i| number_words[i])
        .collect::<Vec<&str>>()
        .join(" ")
}

fn main() {
    let input_string = "three two five zero".to_string();
    let sorted_string = sort_numbers(input_string);
    println!("{sorted_string}"); // Output should be: "zero two three five"
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
