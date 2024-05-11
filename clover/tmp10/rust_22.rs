
use std::any::Any;

fn filter_integers(values: Vec<Box<dyn Any>>) -> Vec<i32> {
    values.into_iter()
        .filter_map(|value| value.downcast::<i32>().ok())
        .map(|value| *value)
        .collect()
}

// Example usage:
// This won't actually run in a playground, since dyn Any does not implement the std::fmt::Debug trait. 
// But you can use the following as a guide to how this function would be used.
//
// fn main() {
//     let values: Vec<Box<dyn Any>> = vec![
//         Box::new(10),
//         Box::new("Hello"),
//         Box::new(20),
//         Box::new(3.14),
//     ];
//     let integers = filter_integers(values);
//     println!("{:?}", integers); // This should print `[10, 20]` if it could run
// }

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_filter_integers() {
        assert_eq!(filter_integers(vec![]), vec![]);
        let v_empty: Vec<Box<dyn Any>> = vec![];
        assert_eq!(
            filter_integers(vec![
                Box::new(4),
                Box::new(v_empty),
                Box::new(23.2),
                Box::new(9),
                Box::new(String::from("adasd"))
            ]),
            vec![4, 9]
        );
        assert_eq!(
            filter_integers(vec![
                Box::new(3),
                Box::new('c'),
                Box::new(3),
                Box::new(3),
                Box::new('a'),
                Box::new('b')
            ]),
            vec![3, 3, 3]
        );
    }

}
