
fn simplify(x: &str, n: &str) -> bool {
    let x_parts: Vec<i64> = x.split('/')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    let n_parts: Vec<i64> = n.split('/')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let numerator = x_parts[0] * n_parts[0];
    let denominator = x_parts[1] * n_parts[1];
    numerator % denominator == 0
}

fn main() {
    // You can test the function here
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
