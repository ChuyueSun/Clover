 the Rust program that implements the functionality described in the docstring:

```rust
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
    let test_string = "Hello, World!".to_string();
    let flipped_case_string = flip_case(test_string);
    println!("{}", flipped_case_string); // prints "hELLO, wORLD!"
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
