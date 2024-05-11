
fn f(n: i32) -> Vec<i64> {
    (1..=n).map(|i| {
        if i % 2 == 0 {
            (1..=i).fold(1, |acc, x| acc * x as i64)
        } else {
            (1..=i).fold(0, |acc, x| acc + x as i64)
        }
    }).collect()
}

fn main() {
    let result = f(10);
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
