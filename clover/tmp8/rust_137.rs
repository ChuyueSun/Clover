
use std::any::Any;
use std::str::FromStr;

fn any_to_f64(a: &dyn Any) -> Option<f64> {
    if let Some(&int_val) = a.downcast_ref::<i32>() {
        Some(int_val as f64)
    } else if let Some(&float_val) = a.downcast_ref::<f64>() {
        Some(float_val)
    } else if let Some(string_val) = a.downcast_ref::<String>() {
        let parsed_string = string_val.replace(',', ".");
        f64::from_str(&parsed_string).ok()
    } else {
        None
    }
}

fn get_larger_of_any<'a, 'b>(a: &'a (dyn Any + 'a), b: &'b (dyn Any + 'b)) -> Option<Box<dyn Any>> {
    let a_f64 = any_to_f64(a);
    let b_f64 = any_to_f64(b);

    match (a_f64, b_f64) {
        (Some(a_val), Some(b_val)) if a_val > b_val => a.downcast_ref::<Any>().map(|v| Box::new(*v)),
        (Some(a_val), Some(b_val)) if b_val > a_val => b.downcast_ref::<Any>().map(|v| Box::new(*v)),
        _ => None,
    }
}

fn main() {
    // Example usage:
    let a: i32 = 10;
    let b: f64 = 20.0;
    let result = get_larger_of_any(&a, &b);
    println!("{:?}", result);
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
