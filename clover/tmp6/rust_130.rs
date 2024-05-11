
fn main() {
    // Example usage:
    let sequence = tri(10);
    println!("{:?}", sequence);
}

fn tri(n: i32) -> Vec<i32> {
    let mut sequence = vec![0; (n + 1) as usize];

    // Base case definitions
    sequence[1] = 3;
    if n == 0 {
        return sequence[..=n as usize].to_vec();
    }

    for i in 2..=n as usize {
        if i % 2 == 0 {
            sequence[i] = 1 + i as i32 / 2;
        } else { 
            // To avoid underflow, ensure we don't access index -1 when i == 1.
            let prev1 = if i >= 2 { sequence[i - 1] } else { 0 };
            let prev2 = if i >= 3 { sequence[i - 2] } else { 0 };
            // To avoid out-of-bounds, ensure we don't access beyond the current sequence size.
            let next1 = if i + 1 <= n as usize { sequence[i + 1] } else { 3 }; // Based on the pattern, the next odd will always be 3
            sequence[i] = prev1 + prev2 + next1;
        }
    }
    sequence
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
