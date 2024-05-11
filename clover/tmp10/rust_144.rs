
fn simplify(x: &str, n: &str) -> bool {
    let parse_fraction = |frac: &str| {
        let parts: Vec<_> = frac.split('/').collect();
        (
            parts[0].parse::<i64>().unwrap(),
            parts[1].parse::<i64>().unwrap(),
        )
    };

    let (num_x, denom_x) = parse_fraction(x);
    let (num_n, denom_n) = parse_fraction(n);

    let new_num = num_x * num_n;
    let new_denom = denom_x * denom_n;

    new_num % new_denom == 0
}

fn main() {
    // Example usage:
    let x = "1/2";
    let n = "3/6";
    println!("{}", simplify(x, n)); // Should print `true` since 1/2 * 3/6 = 1/4 which is a whole number
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
