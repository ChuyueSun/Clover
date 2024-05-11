
fn truncate_number(number: &f32) -> f32 {
    number - number.floor()
}

fn main() {
    let number: f32 = 123.456;
    println!("Decimal part: {}", truncate_number(&number));
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
