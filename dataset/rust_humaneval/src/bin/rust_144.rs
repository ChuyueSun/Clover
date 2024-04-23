fn main() {}

/*
Your task is to implement a function that will simplify the expression
    x * n. The function returns True if x * n evaluates to a whole number and False
    otherwise. Both x and n, are string representation of a fraction, and have the following format,
    <numerator>/<denominator> where both numerator and denominator are positive whole numbers.

    You can assume that x, and n are valid fractions, and do not have zero as denominator.

*/

fn simplify(x: &str, n: &str) -> bool {
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut d = 0;
    let _i = 0;
    for i in 0..x.len() {
        if x.chars().nth(i).unwrap() == '/' {
            a = x
                .chars()
                .take(i)
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
            b = x
                .chars()
                .skip(i + 1)
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
        }
    }
    for i in 0..n.len() {
        if n.chars().nth(i).unwrap() == '/' {
            c = n
                .chars()
                .take(i)
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
            d = n
                .chars()
                .skip(i + 1)
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
        }
    }
    if (a * c) % (b * d) == 0 {
        return true;
    }
    return false;
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
