fn main() {}

/*

    Given two positive integers a and b, return the even digits between a
    and b, in ascending order.

*/

fn generate_integers(a: i32, b: i32) -> Vec<i32> {
    let mut a = a;
    let mut b = b;
    let m;

    if b < a {
        m = a;
        a = b;
        b = m;
    }

    let mut out = vec![];
    for i in a..=b {
        if i < 10 && i % 2 == 0 {
            out.push(i);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_integers() {
        assert_eq!(generate_integers(2, 10), vec![2, 4, 6, 8]);
        assert_eq!(generate_integers(10, 2), vec![2, 4, 6, 8]);
        assert_eq!(generate_integers(132, 2), vec![2, 4, 6, 8]);
        assert_eq!(generate_integers(17, 89), vec![]);
    }
}
