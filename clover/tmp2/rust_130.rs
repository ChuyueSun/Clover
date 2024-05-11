
fn tri(n: i32) -> Vec<i32> {
    let mut tribonacci: Vec<i32> = vec![3]; // tri(1) = 3

    for i in 2..=n + 1 {
        let value = if i % 2 == 0 {
            1 + i / 2
        } else {
            let prev1 = tribonacci[(i - 2) as usize - 1];
            let prev2 = tribonacci[(i - 3) as usize - 1];
            let next1 = 1 + (i + 1) / 2;

            prev1 + prev2 + next1
        };
        tribonacci.push(value);
    }

    tribonacci
}

fn main() {
    // This main function is for illustration purposes and testing the tri function.
    let n = 10; // Can be any non-negative integer.
    let trib_seq = tri(n);
    println!("{:?}", trib_seq);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tri() {
        assert!(tri(3) == vec![1, 3, 2, 8]);
        assert!(tri(4) == vec![1, 3, 2, 8, 3]);
        assert!(tri(5) == vec![1, 3, 2, 8, 3, 15]);
        assert!(tri(6) == vec![1, 3, 2, 8, 3, 15, 4]);
        assert!(tri(7) == vec![1, 3, 2, 8, 3, 15, 4, 24]);
        assert!(tri(8) == vec![1, 3, 2, 8, 3, 15, 4, 24, 5]);
        assert!(tri(9) == vec![1, 3, 2, 8, 3, 15, 4, 24, 5, 35]);
        assert!(
            tri(20)
                == vec![1, 3, 2, 8, 3, 15, 4, 24, 5, 35, 6, 48, 7, 63, 8, 80, 9, 99, 10, 120, 11]
        );
        assert!(tri(0) == vec![1]);
        assert!(tri(1) == vec![1, 3]);
    }

}
