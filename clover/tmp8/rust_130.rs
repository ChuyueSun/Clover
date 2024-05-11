
fn tri(n: i32) -> Vec<i32> {
    let mut sequence = vec![0; (n + 1) as usize];
    if n == 0 {
        sequence[0] = 3;
    } else if n >= 1 {
        sequence[0] = 3;
        for i in 1..=n as usize {
            if i % 2 == 0 {
                sequence[i] = 1 + i as i32 / 2;
            } else {
                if i+1 <= n as usize {
                    sequence[i+1] = 1 + (i+1) as i32 / 2;
                }
                sequence[i] = sequence[i - 1] + sequence[i - 2] + sequence[i + 1];
            }
        }
    }
    sequence
}

fn main() {
    println!("{:?}", tri(0)); // Should print [3]
    println!("{:?}", tri(1)); // Should print [3, 2]
    println!("{:?}", tri(2)); // Should print [3, 2, 2]
    println!("{:?}", tri(3)); // Should print [3, 2, 2, 8]
    println!("{:?}", tri(4)); // Should print [3, 2, 2, 8, 3]
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
