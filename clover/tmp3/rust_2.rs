
/// Returns the decimal part of the given positive floating point number.
///
/// # Arguments
/// * `number` - A positive floating point number to be decomposed.
///
/// # Returns
/// * A `f32` representing the decimal part of the number,
///   which is the number after removing its integer part.
fn truncate_number(number: &f32) -> f32 {
    number - number.floor()
}

fn main() {
    // Example usage
    let num = 5.678;
    println!("The decimal part of {} is {}", num, truncate_number(&num));
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_truncate_number() {
        assert_eq!(truncate_number(&3.5), 0.5);
        let t1: f32 = 1.33 - 0.33;
        assert!(truncate_number(&t1) < 0.000001);
        let t2: f32 = 123.456 - 0.456;
        assert!(truncate_number(&t2) < 0.000001);
    }

}
