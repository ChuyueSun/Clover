
use std::any::Any;
use std::str::FromStr;

fn any_to_f64(a: &dyn Any) -> Option<f64> {
    if let Some(&i) = a.downcast_ref::<i32>() {
        Some(i as f64)
    } else if let Some(&f) = a.downcast_ref::<f64>() {
        Some(f)
    } else if let Some(s) = a.downcast_ref::<String>() {
        let fixed_string = s.replace(",", ".");
        f64::from_str(&fixed_string).ok()
    } else {
        None
    }
}

fn main() {
    let a = 10;
    let b = 10.5;
    let c = "10.5";
    let d = "10,5";

    println!("{:?}", any_to_f64(&a as &dyn Any)); // Should print: Some(10.0)
    println!("{:?}", any_to_f64(&b as &dyn Any)); // Should print: Some(10.5)
    println!("{:?}", any_to_f64(&c as &dyn Any)); // Should print: Some(10.5)
    println!("{:?}", any_to_f64(&d as &dyn Any)); // Should print: Some(10.5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_one() {
        assert_eq!(compare_one(&1, &2), RtnType::Int(2));
        assert_eq!(compare_one(&1, &2.5), RtnType::Float(2.5));
        assert_eq!(compare_one(&2, &3), RtnType::Int(3));
        assert_eq!(compare_one(&5, &6), RtnType::Int(6));
        assert_eq!(compare_one(&1, &"2.3"), RtnType::String("2.3".to_string()));
        assert_eq!(compare_one(&"5.1", &"6"), RtnType::String("6".to_string()));
        assert_eq!(compare_one(&"1", &"2"), RtnType::String("2".to_string()));
        assert_eq!(compare_one(&"1", &1), RtnType::String("None".to_string()));
    }

}
