
use std::any::Any;

fn filter_integers(values: Vec<Box<dyn Any>>) -> Vec<i32> {
    values.into_iter()
        .filter_map(|value| value.downcast::<i32>().ok().map(|v| *v))
        .collect()
}

fn main() {
    // Test the function with sample data
    let values: Vec<Box<dyn Any>> = vec![
        Box::new(10),
        Box::new(20),
        Box::new("A string".to_string()),
        Box::new(30),
    ];

    let integers = filter_integers(values);
    assert_eq!(integers, vec![10, 20, 30]);
    println!("Filtered integers: {:?}", integers);
}

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