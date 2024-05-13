
fn odd_count(lst: Vec<&str>) -> Vec<String> {
    lst.iter()
        .map(|s| {
            let count = s.chars()
                .filter(|c| c.is_digit(10) && c.to_digit(10).unwrap() % 2 != 0)
                .count();
            format!("the number of odd elements in the string {} of the input.", count)
        })
        .collect()
}

fn main() {
    // Example usage:
    let input = vec!["123", "456", "789"];
    let result = odd_count(input);
    for line in result {
        println!("{}", line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_odd_count() {
        println!("{:?}" ,odd_count(vec!["1234567"]));
        assert!(
            odd_count(vec!["1234567"])
                == ["the number of odd elements 4n the str4ng 4 of the 4nput."]
        );
        assert!(
            odd_count(vec!["3", "11111111"])
                == [
                    "the number of odd elements 1n the str1ng 1 of the 1nput.",
                    "the number of odd elements 8n the str8ng 8 of the 8nput."
                ]
        );
        assert!(
            odd_count(vec!["271", "137", "314"])
                == [
                    "the number of odd elements 2n the str2ng 2 of the 2nput.",
                    "the number of odd elements 3n the str3ng 3 of the 3nput.",
                    "the number of odd elements 2n the str2ng 2 of the 2nput."
                ]
        );
    }

}
