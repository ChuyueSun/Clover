
fn solve_161(s: &str) -> String {
    let mut has_letters = false;
    let mut result = String::new();

    for c in s.chars() {
        if c.is_alphabetic() {
            has_letters = true;
            result.push(if c.is_uppercase() { c.to_lowercase().next().unwrap() } else { c.to_uppercase().next().unwrap() });
        } else {
            result.push(c);
        }
    }

    if !has_letters {
        result.chars().rev().collect()
    } else {
        result
    }
}

fn main() {
    // Example usage
    let input = "Hello, World!";
    let output = solve_161(input);
    println!("{}", output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_161() {
        assert_eq!(solve_161("AsDf"), "aSdF");
        assert_eq!(solve_161("1234"), "4321");
        assert_eq!(solve_161("ab"), "AB");
        assert_eq!(solve_161("#a@C"), "#A@c");
        assert_eq!(solve_161("#AsdfW^45"), "#aSDFw^45");
        assert_eq!(solve_161("#6@2"), "2@6#");
        assert_eq!(solve_161("#$a^D"), "#$A^d");
        assert_eq!(solve_161("#ccc"), "#CCC");
    }

}
