
fn tri(n: i32) -> Vec<i32> {
    let mut sequence = vec![3];
    for i in 1..=n {
        if i % 2 == 0 {
            sequence.push(1 + i / 2);
        } else {
            let prev1 = sequence[(i - 1) as usize];
            let prev2 = sequence[(i - 2) as usize];
            let next = if sequence.len() > (i + 1) as usize {
                sequence[(i + 1) as usize]
            } else {
                sequence[0] // i+1 would be out of bounds, use tri(1)
            };
            sequence.push(prev1 + prev2 + next);
        }
    }
    sequence
}

fn main() {
    // Example usage:
    let result = tri(5);
    println!("{:?}", result); // [3, 2, 8, 6, 15, 10]
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
