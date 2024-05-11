
fn truncate_number(number: f32) -> f32 {
    number.fract()
}

fn main() {
    let num = 123.456;
    let decimal_part = truncate_number(num);
    println!("The decimal part is: {}", decimal_part);
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
