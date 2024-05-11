
use std::any::Any;
use std::str::FromStr;

fn any_to_f64(a: &dyn Any) -> Option<f64> {
    if let Some(&i) = a.downcast_ref::<i32>() {
        Some(i as f64)
    } else if let Some(&f) = a.downcast_ref::<f64>() {
        Some(f)
    } else if let Some(s) = a.downcast_ref::<String>() {
        let normalized = s.replace(',', ".");
        f64::from_str(&normalized).ok()
    } else {
        None
    }
}

fn compare(a: &dyn Any, b: &dyn Any) -> Option<Box<dyn Any>> {
    let a_val = any_to_f64(a)?;
    let b_val = any_to_f64(b)?;

    if (a_val - b_val).abs() < f64::EPSILON {
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

impl<T> BoxClone for T
where
    T: 'static + Any + Clone,
{
    fn box_clone(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Any> {
    fn clone(&self) -> Box<dyn Any> {
        self.box_clone()
    }
}

fn main() {
    let num1: Box<dyn Any> = Box::new(10.5);
    let num2: Box<dyn Any> = Box::new("20,0".to_string());

    match compare(&*num1, &*num2) {
        Some(larger) => {
            if let Some(val) = larger.downcast_ref::<f64>() {
                println!("Larger is f64: {}", val);
            } else if let Some(val) = larger.downcast_ref::<i32>() {
                println!("Larger is i32: {}", val);
            } else if let Some(val) = larger.downcast_ref::<String>() {
                println!("Larger is String: {}", val);
            }
        }
        None => println!("Values are equal"),
    }
}
```
The main function and the usage of `compare` function is shown as an example. The `BoxClone` trait and its implementation, along with the `Clone` implementation for `Box<dyn Any>`, is required to return the larger variable in its given type from the `compare` function. If the actual application doesn't require ownership of the returned value to be transferred, these parts can be omitted and references can be used instead to avoid the complexity of cloning `Box<dyn Any
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
