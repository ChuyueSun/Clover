
fn simplify(x: &str, n: &str) -> bool {
    let mut parts_x: Vec<i64> = x.split('/').map(|s| s.parse::<i64>().unwrap()).collect();
    let mut parts_n: Vec<i64> = n.split('/').map(|s| s.parse::<i64>().unwrap()).collect();

    // Cross multiply to combine the fractions
    let numerator = parts_x[0] * parts_n[0];
    let denominator = parts_x[1] * parts_n[1];

    // Simplify the fraction
    let gcd = gcd(numerator, denominator);
    let simplified_numerator = numerator / gcd;
    let simplified_denominator = denominator / gcd;

    simplified_denominator == 1
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a.abs()
}

fn main() {
    // Example usage
    println!("{}", simplify("1/2", "2/1")); // true
    println!("{}", simplify("1/3", "3/4")); // false
    println!("{}", simplify("2/5", "5/2")); // true
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
