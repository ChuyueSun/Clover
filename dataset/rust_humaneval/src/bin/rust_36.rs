fn main() {}

/*
Return the number of times the digit 7 appears in integers less than n which are divisible by 11 or 13.

*/

fn fizz_buzz(n: i32) -> i32 {
    let mut ns: Vec<i32> = vec![];

    for i in 0..n {
        if i % 11 == 0 || i % 13 == 0 {
            ns.push(i);
        }
    }

    let s: String = ns
        .into_iter()
        .fold(String::new(), |s: String, n: i32| s + &n.to_string());
    let mut ans: i32 = 0;

    for c in s.chars() {
        if c == '7' {
            ans += 1;
        }
    }
    return ans;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizz_buzz() {
        assert!(fizz_buzz(50) == 0);
        assert!(fizz_buzz(78) == 2);
        assert!(fizz_buzz(79) == 3);
        assert!(fizz_buzz(100) == 3);
        assert!(fizz_buzz(200) == 6);
        assert!(fizz_buzz(4000) == 192);
        assert!(fizz_buzz(10000) == 639);
        assert!(fizz_buzz(100000) == 8026);
    }
}
