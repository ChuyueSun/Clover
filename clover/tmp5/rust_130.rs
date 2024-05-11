
fn tri(n: i32) -> Vec<i32> {
    if n < 0 {
        return vec![];
    }
    
    let mut sequence = vec![3];
    
    for i in 1..=n {
        if i % 2 == 0 {
            sequence.push(1 + i / 2);
        } else {
            let len = sequence.len();
            let next_val = if i == 1 {
                sequence[len - 1]
            } else if i == 2 {
                sequence[len - 1] + sequence[len - 2]
            } else {
                sequence[len - 1] + sequence[len - 2] + sequence[0]
            };
            sequence.push(next_val);
        }
    }
    
    sequence
}

fn main() {
    // Example usage:
    let result = tri(10);
    println!("{:?}", result);
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
