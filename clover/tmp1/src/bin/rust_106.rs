
fn f(n: i32) -> Vec<i32> {
    let mut result = vec![];
    for i in 1..=n {
        if i % 2 == 0 {
            result.push((1..=i).product());
        } else {
            result.push((1..=i).sum());
        }
    }
    result
}

fn main() {
    let n = 5;
    println!("{:?}", f(n));
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
