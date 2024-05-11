
fn tri(n: i32) -> Vec<i32> {
    let mut sequence = vec![0; (n + 1) as usize];
    sequence[0] = 3;
    if n == 0 {
        return sequence;
    }
    sequence[1] = 3;
    for i in 2..=n as usize {
        if i % 2 == 0 {
            sequence[i] = 1 + i as i32 / 2;
        } else {
            sequence[i] = sequence[i - 1] + sequence[i - 2] + sequence[i.saturating_sub(3)];
        }
    }
    sequence
}

fn main() {
    // Example usage
    println!("{:?}", tri(5));
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
