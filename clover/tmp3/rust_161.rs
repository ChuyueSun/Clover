
fn solve_161(s: &str) -> String {
    if s.chars().any(|c| c.is_alphabetic()) {
        s.chars()
            .map(|c| {
                if c.is_lowercase() {
                    c.to_ascii_uppercase()
                } else if c.is_uppercase() {
                    c.to_ascii_lowercase()
                } else {
                    c
                }
            })
            .collect()
    } else {
        s.chars().rev().collect()
    }
}

fn main() {
    // Example usage:
    let input = "a1b2C3";
    let result = solve_161(input);
    println!("{}", result); // Should print "A1B2c3"
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
