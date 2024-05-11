
fn odd_count(lst: Vec<&str>) -> Vec<String> {
    lst.into_iter()
        .map(|s| {
            let count = s.chars()
                .filter(|&c| c == '1' || c == '3' || c == '5' || c == '7' || c == '9')
                .count();
            format!("the number of odd elements in the string {} of the input.", count)
        })
        .collect()
}

fn main() {
    // Example usage:
    let strings = vec!["12345", "67890", "2468"];
    let result = odd_count(strings);
    for line in result {
        println!("{}", line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_odd_count() {
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
