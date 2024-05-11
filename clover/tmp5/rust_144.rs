
fn simplify(x: &str, n: &str) -> bool {
    let frac_x: Vec<&str> = x.split('/').collect();
    let frac_n: Vec<&str> = n.split('/').collect();

    let num_x = frac_x[0].parse::<i64>().unwrap();
    let den_x = frac_x[1].parse::<i64>().unwrap();
    let num_n = frac_n[0].parse::<i64>().unwrap();
    let den_n = frac_n[1].parse::<i64>().unwrap();

    num_x * num_n % (den_x * den_n) == 0
}

fn main() {
    // Example usage
    println!("{}", simplify("1/2", "2/1")); // True
    println!("{}", simplify("3/4", "4/3")); // False
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
