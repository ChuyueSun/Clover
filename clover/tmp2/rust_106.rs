
fn f(n: i32) -> Vec<i64> {
    (1..=n).map(|i| {
        if i % 2 == 0 {
            (1..=i).fold(1, |acc, x| acc * x)
        } else {
            (1..=i).sum()
        }
    }).collect()
}

fn main() {
    let n = 10;
    let result = f(n);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f() {
        assert!(f(5) == vec![1, 2, 6, 24, 15]);
        assert!(f(7) == vec![1, 2, 6, 24, 15, 720, 28]);
        assert!(f(1) == vec![1]);
        assert!(f(3) == vec![1, 2, 6]);
    }

}
