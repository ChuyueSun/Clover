
fn intersperse(numbers: Vec<u32>, delimiter: u32) -> Vec<u32> {
    if numbers.len() <= 1 {
        return numbers;
    }

    let mut result = Vec::with_capacity(numbers.len() * 2 - 1);
    let mut iter = numbers.iter();

    if let Some(first) = iter.next() {
        result.push(*first);
    }

    for number in iter {
        result.push(delimiter);
        result.push(*number);
    }

    result
}

fn main() {
    let numbers = vec![1, 2, 3, 4];
    let delimiter = 7;
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
