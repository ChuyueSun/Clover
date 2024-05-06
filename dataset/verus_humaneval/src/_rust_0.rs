fn main() {}
use vstd::{prelude::*};
/*
 Check if in given list of numbers, are any two numbers closer to each other than
    given threshold.

*/
verus! {
fn has_close_elements(numbers: Vec<f32>, threshold: f32) -> (r: bool)
    ensures
        r == (exists |i: int, j: int| 0 <= i < numbers.len() && 0 <= j < numbers.len() && i != j && (numbers[i] - numbers[j]).abs() < threshold)
{
    for i in 0..numbers.len() {
        for j in 1..numbers.len() {
            if i != j {
                let distance: f32 = numbers[i] - numbers[j];

                if distance.abs() < threshold {
                    return true;
                }
            }
        }
    }

    return false;
}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_close_elements() {
        assert_eq!(
            has_close_elements(vec![11.0, 2.0, 3.9, 4.0, 5.0, 2.2], 0.3),
            true
        );
        assert_eq!(
            has_close_elements(vec![1.0, 2.0, 3.9, 4.0, 5.0, 2.2], 0.05),
            false
        );
        assert_eq!(
            has_close_elements(vec![1.0, 2.0, 5.9, 4.0, 5.0], 0.95),
            true
        );
        assert_eq!(
            has_close_elements(vec![1.0, 2.0, 5.9, 4.0, 5.0], 0.8),
            false
        );
        assert_eq!(
            has_close_elements(vec![1.0, 2.0, 3.0, 4.0, 5.0, 2.0], 0.1),
            true
        );
        assert_eq!(has_close_elements(vec![1.1, 2.2, 3.1, 4.1, 5.1], 1.0), true);
        assert_eq!(
            has_close_elements(vec![1.1, 2.2, 3.1, 4.1, 5.1], 0.5),
            false
        );
    }
}
