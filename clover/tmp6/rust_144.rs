
fn simplify(x: &str, n: &str) -> bool {
    let parse_fraction = |s: &str| {
        let parts: Vec<i64> = s.split('/').map(|part| part.parse::<i64>().unwrap()).collect();
        (parts[0], parts[1])
    };

    let (x_num, x_denom) = parse_fraction(x);
    let (n_num, n_denom) = parse_fraction(n);

    x_num * n_num % (x_denom * n_denom) == 0
}

fn main() {
    println!("{}", simplify("1/2", "3/4")); // false
    println!("{}", simplify("2/3", "3/2")); // true
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
