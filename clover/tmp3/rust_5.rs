
fn intersperse(numbers: Vec<u32>, delimiter: u32) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();

    if let Some(first) = numbers.first() {
        result.push(*first);
    }

    for &num in numbers.iter().skip(1) {
        result.push(delimiter);
        result.push(num);
    }

    result
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let delimiter = 0;
    let interspersed = intersperse(numbers, delimiter);
    
    println!("{:?}", interspersed);
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_intersperse() {
        assert!(intersperse(vec![], 7) == vec![]);
        assert!(intersperse(vec![5, 6, 3, 2], 8) == vec![5, 8, 6, 8, 3, 8, 2]);
        assert!(intersperse(vec![2, 2, 2], 2) == vec![2, 2, 2, 2, 2]);
    }

}
