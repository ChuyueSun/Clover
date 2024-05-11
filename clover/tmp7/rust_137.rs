
use std::any::Any;
use std::str::FromStr;

pub fn larger_of<T: PartialOrd>(a: T, b: T) -> Option<T> {
    if a > b {
        Some(a)
    } else if a < b {
        Some(b)
    } else {
        None
    }
}

fn any_to_f64(value: &dyn Any) -> Option<f64> {
    if let Some(&int_val) = value.downcast_ref::<i32>() {
        Some(int_val as f64)
    } else if let Some(&float_val) = value.downcast_ref::<f64>() {
        Some(float_val)
    } else if let Some(str_val) = value.downcast_ref::<String>() {
        let parsed_str = str_val.replace(',', ".");
        f64::from_str(&parsed_str).ok()
    } else {
        None
    }
}

pub fn compare_and_return<T: PartialOrd + 'static>(a: &T, b: &T) -> Option<T> 
where
    T: Clone + FromStr + ToString,
{
    if let (Ok(a_val), Ok(b_val)) = (any_to_f64(a as &dyn Any), any_to_f64(b as &dyn Any)) {
        if a_val != b_val {
            match a_val.partial_cmp(&b_val) {
                Some(std::cmp::Ordering::Greater) => Some(a.clone()),
                Some(std::cmp::Ordering::Less) => Some(b.clone()),
                _ => None,
            }
        } else {
            None
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integers() {
        let a = 10;
        let b = 20;
        assert_eq!(compare_and_return(&a, &b), Some(b));
    }

    #[test]
    fn test_floats() {
        let a = 10.5;
        let b = 20.5;
        assert_eq!(compare_and_return(&a, &b), Some(b));
    }

    #[test]
    fn test_equal_values() {
        let a = 15;
        let b = 15;
        assert_eq!(compare_and_return(&a, &b), None);
    }

    #[test]
    fn test_strings() {
        let a = "10.1".to_string();
        let b = "10,2".to_string(); // comma as decimal point
        assert_eq!(compare_and_return(&a, &b), Some(b));
    }
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
