
use std::any::Any;

fn filter_integers(values: Vec<Box<dyn Any>>) -> Vec<i32> {
    values.into_iter()
        .filter_map(|value| value.downcast::<i32>().ok().map(|box_int| *box_int))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_integers() {
        let values: Vec<Box<dyn Any>> = vec![
            Box::new(10i32),
            Box::new(20i32),
            Box::new("Not an integer"),
            Box::new(30i32),
        ];
        let integers = filter_integers(values);
        assert_eq!(integers, vec![10, 20, 30]);
    }
}

fn main() {
    let values: Vec<Box<dyn Any>> = vec![
        Box::new(10i32),
        Box::new(3.14f64),
        Box::new("Hello, world!"),
        Box::new(42i32),
    ];
    let filtered_values = filter_integers(values);
    for value in filtered_values {
        println!("Filtered integer: {}", value);
    }
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
