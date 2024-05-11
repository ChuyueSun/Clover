
fn sort_numbers(numbers: String) -> String {
    let mut num_vec: Vec<&str> = numbers.split_whitespace().collect();
    let num_order = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut num_tuples = num_vec.iter().map(|&num| {
        num_order.iter().position(|&n| n == num).unwrap_or(usize::MAX)
    }).collect::<Vec<_>>();
    num_tuples.sort_unstable();
    num_tuples.iter().map(|&i| num_order[i]).collect::<Vec<_>>().join(" ")
}

fn main() {
    let input = "three one four two zero".to_string();
    let sorted = sort_numbers(input);
    println!("{}", sorted); // "zero one two three four"
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
