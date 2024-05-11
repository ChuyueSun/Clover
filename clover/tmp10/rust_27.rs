
fn flip_case(string: String) -> String {
    string.chars().map(|c| if c.is_lowercase() {
        c.to_uppercase().collect::<String>()
    } else {
        c.to_lowercase().collect::<String>()
    }).collect()
}

fn main() {
    let example = "HeLLo WOrlD!".to_string();
    let flipped = flip_case(example);
    println!("{}", flipped);
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
