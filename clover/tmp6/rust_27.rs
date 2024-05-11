 the Rust program that implements the functionality described in the docstring:

```rust
fn flip_case(string: String) -> String {
    string.chars().map(|c| {
        if c.is_lowercase() {
            c.to_uppercase().next().unwrap()
        } else if c.is_uppercase() {
            c.to_lowercase().next().unwrap()
        } else {
            c
        }
    }).collect()
}

fn main() {
    let input_string = "RustProgramming123".to_string();
    let flipped_case_string = flip_case(input_string);
    println!("{}", flipped_case_string); // Output will be "rUSTpROGRAMMING123"
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
