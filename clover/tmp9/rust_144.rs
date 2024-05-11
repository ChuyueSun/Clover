
fn simplify(x: &str, n: &str) -> bool {
    let parse_fraction = |s: &str| {
        let parts: Vec<&str> = s.split('/').collect();
        (
            parts[0].parse::<i64>().unwrap(),
            parts[1].parse::<i64>().unwrap(),
        )
    };

    let (num_x, den_x) = parse_fraction(x);
    let (num_n, den_n) = parse_fraction(n);

    // Simplify the multiplication of the two fractions
    let num_product = num_x * num_n;
    let den_product = den_x * den_n;

    // Check if the result is a whole number
    num_product % den_product == 0
}

fn main() {
    assert_eq!(simplify("1/2", "2/1"), true);
    assert_eq!(simplify("1/3", "3/2"), true);
    assert_eq!(simplify("2/3", "3/2"), true);
    assert_eq!(simplify("1/4", "1/2"), false);
    // Add more tests if necessary
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
