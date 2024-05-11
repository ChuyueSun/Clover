the Rust program that implements the functionality described:

```rust
fn flip_case(string: String) -> String {
    string.chars().map(|c| {
        if c.is_lowercase() {
            c.to_ascii_uppercase()
        } else {
            c.to_ascii_lowercase()
        }
    }).collect()
}

fn main() {
    let test_string = "Hello, World!";
    println!("{}", flip_case(test_string.to_string()));
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
