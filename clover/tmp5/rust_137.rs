
use std::any::Any;
use std::str::FromStr;

fn main() {
    // Examples
    let int_a: i32 = 12;
    let int_b: i32 = 24;
    println!("{:?}", get_larger(&int_a, &int_b));

    let float_a: f64 = 12.34;
    let float_b: f64 = 56.78;
    println!("{:?}", get_larger(&float_a, &float_b));

    let string_a: &str = "99.99";
    let string_b: &str = "88.88";
    println!("{:?}", get_larger(&string_a, &string_b));
}

fn get_larger(a: &dyn Any, b: &dyn Any) -> Option<Box<dyn Any>> {
    if let Some(&int_a) = a.downcast_ref::<i32>() {
        if let Some(&int_b) = b.downcast_ref::<i32>() {
            return if int_a > int_b { Some(Box::new(int_a)) } else if int_a < int_b { Some(Box::new(int_b)) } else { None };
        }
    }

    if let Some(&float_a) = a.downcast_ref::<f64>() {
        if let Some(&float_b) = b.downcast_ref::<f64>() {
            return if float_a > float_b { Some(Box::new(float_a)) } else if float_a < float_b { Some(Box::new(float_b)) } else { None };
        }
    }

    if let Some(string_a) = a.downcast_ref::<&str>() {
        if let Some(string_b) = b.downcast_ref::<&str>() {
            let num_a = string_a.replace(',', ".").parse::<f64>().ok();
            let num_b = string_b.replace(',', ".").parse::<f64>().ok();
            match (num_a, num_b) {
                (Some(a), Some(b)) => {
                    return if a > b { Some(Box::new(a.to_string())) } else if a < b { Some(Box::new(b.to_string())) } else { None }
                }
                _ => {}
            }
        }
    }

    None
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
