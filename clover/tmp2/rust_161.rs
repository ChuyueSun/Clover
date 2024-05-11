
fn solve_161(s: &str) -> String {
    if s.chars().all(|c| !c.is_alphabetic()) {
        s.chars().rev().collect()
    } else {
        s.chars()
            .map(|c| {
                if c.is_uppercase() {
                    c.to_lowercase().next().unwrap()
                } else if c.is_lowercase() {
                    c.to_uppercase().next().unwrap()
                } else {
                    c
                }
            })
            .collect()
    }
}

fn main() {
    // Example usage:
    let input = "Rust!";
    let result = solve_161(input);
    println!("{}", result); // Output: rUST!
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
