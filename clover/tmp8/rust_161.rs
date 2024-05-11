
fn solve_161(s: &str) -> String {
    if s.chars().all(|c| !c.is_alphabetic()) {
        return s.chars().rev().collect();
    }
    s.chars()
        .map(|c| {
            if c.is_alphabetic() {
                if c.is_lowercase() {
                    c.to_ascii_uppercase()
                } else {
                    c.to_ascii_lowercase()
                }
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    // Example usage:
    let input = "Hello World!";
    println!("{}", solve_161(input)); // Output: "hELLO wORLD!"
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
