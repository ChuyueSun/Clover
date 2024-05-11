
fn simplify(x: &str, n: &str) -> bool {
    let parse_fraction = |s: &str| {
        let parts: Vec<&str> = s.split('/').collect();
        (
            parts[0].parse::<i64>().unwrap(),
            parts[1].parse::<i64>().unwrap(),
        )
    };

    let (numerator_x, denominator_x) = parse_fraction(x);
    let (numerator_n, denominator_n) = parse_fraction(n);

    let numerator_product = numerator_x * numerator_n;
    let denominator_product = denominator_x * denominator_n;

    numerator_product % denominator_product == 0
}

fn main() {
    // Examples
    println!("{}", simplify("1/2", "2/1")); // true
    println!("{}", simplify("2/3", "3/4")); // false
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
