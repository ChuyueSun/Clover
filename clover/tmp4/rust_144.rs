
fn simplify(x: &str, n: &str) -> bool {
    use std::str::FromStr;

    fn gcd(a: i64, b: i64) -> i64 {
        match b {
            0 => a,
            _ => gcd(b, a % b),
        }
    }

    let parse_fraction = |frac: &str| -> (i64, i64) {
        let mut parts = frac.split('/');
        (
            i64::from_str(parts.next().unwrap()).unwrap(),
            i64::from_str(parts.next().unwrap()).unwrap(),
        )
    };

    let (num_x, den_x) = parse_fraction(x);
    let (num_n, den_n) = parse_fraction(n);
    let numerator = num_x * num_n;
    let denominator = den_x * den_n;

    gcd(numerator, denominator) == denominator
}

fn main() {
    // Example usage:
    let result = simplify("1/2", "2/3");
    println!("The result is: {}", result);
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
