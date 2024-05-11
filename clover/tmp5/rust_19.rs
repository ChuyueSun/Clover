
fn sort_numbers(numbers: String) -> String {
    let number_to_digit = |word: &str| match word {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 10, // handle unexpected input
    };

    let digit_to_number = |digit: u32| match digit {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => "", // handle unexpected input
    };

    let mut words = numbers.split_whitespace().collect::<Vec<&str>>();
    words.sort_unstable_by_key(|&word| number_to_digit(word));

    words
        .iter()
        .map(|&word| digit_to_number(number_to_digit(word)))
        .collect::<Vec<&str>>()
        .join(" ")
}

fn main() {
    let numbers = "three one four two zero".to_string();
    let sorted_numbers = sort_numbers(numbers);
    println!("{}", sorted_numbers); // Should output: "zero one two three four"
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
