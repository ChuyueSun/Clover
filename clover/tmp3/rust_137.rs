
use std::any::Any;
use std::str::FromStr;

fn larger_of<T: PartialOrd + Copy>(a: T, b: T) -> Option<T> {
    if a > b {
        Some(a)
    } else if a < b {
        Some(b)
    } else {
        None
    }
}

fn Any_to_f64(a: &dyn Any) -> Option<f64> {
    a.downcast_ref::<i32>().map(|v| *v as f64)
    .or_else(|| a.downcast_ref::<f64>().copied())
    .or_else(|| a.downcast_ref::<String>().and_then(|s|
                            f64::from_str(&s.replace(",", ".")).ok()))
}

fn main() {
    // Examples
    let int_5: Box<dyn Any> = Box::new(5i32);
    let float_3_14: Box<dyn Any> = Box::new(3.14f64);
    let string_num: Box<dyn Any> = Box::new("2.718".to_string());
    let string_num_comma: Box<dyn Any> = Box::new("3,1415".to_string());
    
    let v1 = Any_to_f64(int_5.as_ref());
    let v2 = Any_to_f64(float_3_14.as_ref());
    let v3 = Any_to_f64(string_num.as_ref());
    let v4 = Any_to_f64(string_num_comma.as_ref());
    
    println!("{:?}, {:?}, {:?}, {:?}", v1, v2, v3, v4);
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
