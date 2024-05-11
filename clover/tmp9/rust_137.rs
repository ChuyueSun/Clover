
use std::any::Any;
use std::str::FromStr;

fn any_to_f64(a: &dyn Any) -> Option<f64> {
    if let Some(&i) = a.downcast_ref::<i32>() {
        Some(i as f64)
    } else if let Some(&f) = a.downcast_ref::<f64>() {
        Some(f)
    } else if let Some(s) = a.downcast_ref::<&str>() {
        s.replace(',', ".")
            .parse::<f64>()
            .ok()
    } else {
        None
    }
}

fn get_larger_value<T: Any + PartialOrd>(a: &T, b: &T) -> Option<T> where T: Copy {
    if a > b {
        Some(*a)
    } else if b > a {
        Some(*b)
    } else {
        None
    }
}

fn main() {
    let a_i: i32 = 5;
    let b_i: i32 = 10;
    let a_f: f64 = 5.5;
    let b_f: f64 = 10.1;
    let a_s: &str = "5.5";
    let b_s: &str = "10,1";

    let a_i_f64 = any_to_f64(&a_i).unwrap();
    let a_f_f64 = any_to_f64(&a_f).unwrap();
    let a_s_f64 = any_to_f64(&a_s).unwrap();

    let b_i_f64 = any_to_f64(&b_i).unwrap();
    let b_f_f64 = any_to_f64(&b_f).unwrap();
    let b_s_f64 = any_to_f64(&b_s).unwrap();

    println!("{:?}", get_larger_value(&a_i_f64, &b_i_f64)); // Some(10.0)
    println!("{:?}", get_larger_value(&a_f_f64, &b_f_f64)); // Some(10.1)
    println!("{:?}", get_larger_value(&a_s_f64, &b_s_f64)); // Some(10.1)
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
