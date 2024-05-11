
fn tri(n: i32) -> Vec<i64> {
    let mut series: Vec<i64> = vec![3];
    
    for i in 1..=n {
        if i % 2 == 0 {
            series.push(1 + i as i64 / 2);
        } else {
            let a = *series.get((i - 1) as usize).unwrap_or(&0);
            let b = *series.get((i - 2) as usize).unwrap_or(&0);
            let c = *series.get(i as usize).unwrap_or(&0);
            series.push(a + b + c);
        }
    }
    
    series
}

fn main() {
    let n = 5;
    let result = tri(n);
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
