
use std::any::Any;
use std::str::FromStr;

fn any_to_f64(a: &dyn Any) -> Option<f64> {
    if let Some(&i) = a.downcast_ref::<i32>() {
        Some(i as f64)
    } else if let Some(&f) = a.downcast_ref::<f32>() {
        Some(f as f64)
    } else if let Some(s) = a.downcast_ref::<&str>() {
        let cleaned_s = s.replace(',', ".");
        f64::from_str(&cleaned_s).ok()
    } else {
        None // The input is not an integer, float, or string representing a real number
    }
}

fn compare_values(a: &dyn Any, b: &dyn Any) -> Option<Box<dyn Any>> {
    let a_val = any_to_f64(a)?;
    let b_val = any_to_f64(b)?;

    if a_val == b_val {
        None
    } else if a_val > b_val {
        Some(a.box_clone())
    } else {
        Some(b.box_clone())
    }
}

trait BoxClone {
    fn box_clone(&self) -> Box<dyn Any>;
}

impl BoxClone for i32 {
    fn box_clone(&self) -> Box<dyn Any> {
        Box::new(*self)
    }
}

impl BoxClone for f32 {
    fn box_clone(&self) -> Box<dyn Any> {
        Box::new(*self)
    }
}

impl BoxClone for &str {
    fn box_clone(&self) -> Box<dyn Any> {
        Box::new(*self)
    }
}

impl<T> BoxClone for T
where
    T: 'static + Clone + Any,
{
    fn box_clone(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }
}

fn main() {
    let a: i32 = 42;
    let b: f32 = 42.1;
    let c: &str = "42.2";
    let d: &str = "42,2"; // This is equivalent to 42.2

    let result = compare_values(&a, &b);
    if let Some(val) = result {
        if let Ok(val) = val.downcast::<f32>() {
            println!("Larger value: {}", val);
        } else if let Ok(val) = val.downcast::<i32>() {
            println!("Larger value: {}", val);
        } else if let Ok(val) = val.downcast::<String>() {
            println!("Larger value: {}", val);
        }
    } else {
        println!("Values are equal or could not be compared.");
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
