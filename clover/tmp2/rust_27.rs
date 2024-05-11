
fn flip_case(string: String) -> String {
    string.chars().map(|c| {
        if c.is_lowercase() {
            c.to_uppercase().to_string()
        } else {
            c.to_lowercase().to_string()
        }
    }).collect()
}

fn main() {
    let example_string = "RuSt Is AmAzInG!".to_string();
    println!("{}", flip_case(example_string));
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
