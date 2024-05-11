
fn sort_numbers(numbers: String) -> String {
    let num_to_int = |s: &str| match s {
        "zero" => Some(0),
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    };

    let mut numbers_vec: Vec<_> = numbers.split_whitespace()
        .filter_map(num_to_int)
        .collect();

    numbers_vec.sort_unstable();

    let int_to_num = |n: &usize| match n {
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
        _ => unreachable!(),
    };

    numbers_vec.into_iter()
        .map(int_to_num)
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() {
    let input = "three one four one five".to_string();
    println!("{}", sort_numbers(input));
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
