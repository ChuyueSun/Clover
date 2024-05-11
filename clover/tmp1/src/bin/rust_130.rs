
fn tri(n: i32) -> Vec<i32> {
    let mut sequence = vec![0; n as usize + 1];

    // Base cases
    sequence[0] = 3;
    if n > 0 {
        sequence[1] = 2;
    }
    if n > 1 {
        sequence[2] = 3;
    }

    // Calculating the sequence values
    for i in 3..=n as usize {
        if i % 2 == 0 {
            sequence[i] = 1 + i as i32 / 2;
        } else {
            sequence[i] = sequence[i - 1] + sequence[i - 2] + sequence[i - 3];
        }
    }

    sequence
}

fn main() {
    // Example usage:
    let n = 7;
    println!("{:?}", tri(n));
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
