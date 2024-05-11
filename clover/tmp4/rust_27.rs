
fn flip_case(string: String) -> String {
    string
        .chars()
        .map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().collect()
            } else {
                c.to_lowercase().collect()
            }
        })
        .collect()
}

fn main() {
    // Example usage:
    let input = "RuStProGramming";
    let flipped = flip_case(input.to_string());
    println!("{}", flipped); // Output: rUsTpROgRAMMING
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_flip_case() {
        assert!(flip_case("".to_string()) == "".to_string());
        assert!(flip_case("Hello!".to_string()) == "hELLO!".to_string());
        assert!(
            flip_case("These violent delights have violent ends".to_string())
                == "tHESE VIOLENT DELIGHTS HAVE VIOLENT ENDS".to_string()
        );
    }

}
