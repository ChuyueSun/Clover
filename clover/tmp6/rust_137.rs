
use std::any::Any;

fn any_to_f64(a: &dyn Any) -> Option<f64> {
    a.downcast_ref::<i32>()
        .map(|&n| n as f64)
        .or_else(|| a.downcast_ref::<f64>().copied())
        .or_else(|| {
            a.downcast_ref::<String>()
                .and_then(|s| s.replace(',', ".").parse::<f64>().ok())
        })
}

fn larger_variable<T: Any + PartialOrd>(a: &T, b: &T) -> Option<&T> {
    if a > b {
        Some(a)
    } else if a < b {
        Some(b)
    } else {
        None
    }
}

fn main() {
    let int1: i32 = 10;
    let int2: i32 = 20;
    let float1: f64 = 10.5;
    let float2: f64 = 20.5;
    let string_num1: String = "30.1".to_string();
    let string_num2: String = "40,2".to_string();

    let larger_int = larger_variable(&int1, &int2);
    let larger_float = larger_variable(&float1, &float2);
    let larger_string = larger_variable(
        &any_to_f64(&string_num1 as &dyn Any).unwrap(),
        &any_to_f64(&string_num2 as &dyn Any).unwrap(),
    );

    println!("Larger int: {:?}", larger_int);
    println!("Larger float: {:?}", larger_float);
    println!("Larger string number: {:?}", larger_string);
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
