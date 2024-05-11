
use std::str::FromStr;

fn simplify(x: &str, n: &str) -> bool {
    let frac_from_str = |frac: &str| {
        let parts: Vec<u64> = frac.split('/').map(|s| u64::from_str(s).unwrap()).collect();
        (parts[0], parts[1])
    };

    let (x_num, x_den) = frac_from_str(x);
    let (n_num, n_den) = frac_from_str(n);

    (x_num * n_num) % (x_den * n_den) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simplify_true() {
        assert!(simplify("1/2", "2/1"));
    }

    #[test]
    fn test_simplify_false() {
        assert!(!simplify("1/3", "2/1"));
    }
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
