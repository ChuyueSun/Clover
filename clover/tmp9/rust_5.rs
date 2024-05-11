
fn intersperse(numbers: Vec<u32>, delimiter: u32) -> Vec<u32> {
    if numbers.is_empty() {
        return vec![];
    }

    let mut interspersed = Vec::with_capacity(numbers.len() * 2 - 1);
    let mut iter = numbers.iter().peekable();

    while let Some(number) = iter.next() {
        interspersed.push(*number);
        if iter.peek().is_some() {
            interspersed.push(delimiter);
        }
    }

    interspersed
}

fn main() {
    let numbers = vec![1, 2, 3, 4];
    let delimiter = 0;
    let interspersed_numbers = intersperse(numbers, delimiter);

    println!("{:?}", interspersed_numbers);
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
