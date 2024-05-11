
fn sort_numbers(numbers: String) -> String {
    let num_map = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut num_vec: Vec<&str> = numbers.split_whitespace().collect();
    num_vec.sort_unstable_by_key(|&num| {
        num_map
            .iter()
            .position(|&n| n == num)
            .expect("Invalid number string")
    });
    num_vec.join(" ")
}

fn main() {
    let numbers = "three one four two zero".to_string();
    println!("{}", sort_numbers(numbers));
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
