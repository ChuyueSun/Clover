
fn simplify(x: &str, n: &str) -> bool {
    // This closure will parse a string fraction and return the numerator and denominator as i64
    let parse_fraction = |fraction: &str| {
        let parts: Vec<i64> = fraction.split('/')
            .map(|p| p.parse::<i64>().unwrap())
            .collect();
        (parts[0], parts[1])
    };

    // Parse both fractions
    let (x_num, x_denom) = parse_fraction(x);
    let (n_num, n_denom) = parse_fraction(n);

    // Multiplied numerators and denominators
    let new_num = x_num * n_num;
    let new_denom = x_denom * n_denom;

    // Check if the resultant numerator is divisible by the resultant denominator
    new_num % new_denom == 0
}

fn main() {
    // Example usage:
    println!("{}", simplify("1/2", "2/1")); // True
    println!("{}", simplify("3/4", "2/3")); // False
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simplify() {
        assert_eq!(simplify("1/5", "5/1"), true);
        assert_eq!(simplify("1/6", "2/1"), false);
        assert_eq!(simplify("5/1", "3/1"), true);
        assert_eq!(simplify("7/10", "10/2"), false);
        assert_eq!(simplify("2/10", "50/10"), true);
        assert_eq!(simplify("7/2", "4/2"), true);
        assert_eq!(simplify("11/6", "6/1"), true);
        assert_eq!(simplify("2/3", "5/2"), false);
        assert_eq!(simplify("5/2", "3/5"), false);
        assert_eq!(simplify("2/4", "8/4"), true);
        assert_eq!(simplify("2/4", "4/2"), true);
        assert_eq!(simplify("1/5", "5/1"), true);
        assert_eq!(simplify("1/5", "1/5"), false);
    }

}
