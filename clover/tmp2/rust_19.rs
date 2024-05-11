
fn sort_numbers(numbers: String) -> String {
    let number_map = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut nums: Vec<&str> = numbers.split_whitespace().collect();
    
    nums.sort_by_key(|&num| number_map.iter().position(|&n| n == num));
    
    nums.join(" ")
}

fn main() {
    // Example usage:
    let input = "three one four two".to_string();
    let sorted = sort_numbers(input);
    println!("{}", sorted); // It should print out "one two three four"
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
