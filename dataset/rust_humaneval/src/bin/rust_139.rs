fn main() {}

/*
The Brazilian factorial is defined as:
    brazilian_factorial(n) = n! * (n-1)! * (n-2)! * ... * 1!
    where n > 0

    The function will receive an integer as input and should return the special
    factorial of this integer.

*/

fn special_factorial(n: i32) -> i64 {
    let mut fact = 1;
    let mut bfact: i64 = 1;
    for i in 1..=n {
        fact = fact * i;
        bfact = bfact * fact as i64;
    }
    bfact
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_special_factorial() {
        assert_eq!(special_factorial(4), 288);
        assert_eq!(special_factorial(5), 34560);
        assert_eq!(special_factorial(7), 125411328000);
        assert_eq!(special_factorial(1), 1);
    }
}
