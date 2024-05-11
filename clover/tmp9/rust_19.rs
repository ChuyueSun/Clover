
fn sort_numbers(numbers: String) -> String {
    let mut nums_as_words = vec![
        "zero", "one", "two", "three", "four", 
        "five", "six", "seven", "eight", "nine"
    ];

    let mut nums = numbers
        .split_whitespace()
        .filter_map(|word| nums_as_words.iter().position(|&n| n == word))
        .collect::<Vec<_>>();

    nums.sort_unstable();
    
    nums.iter()
        .map(|&i| nums_as_words[i])
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() {
    let input = "three one four two".to_string();
    let sorted = sort_numbers(input);
    println!("{}", sorted);
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
