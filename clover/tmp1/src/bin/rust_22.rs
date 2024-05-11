
use std::any::Any;

fn filter_integers(values: Vec<Box<dyn Any>>) -> Vec<i32> {
    values.into_iter()
        .filter_map(|value| value.downcast::<i32>().ok().map(|boxed_int| *boxed_int))
        .collect()
}

fn main() {
    // Example usage:
    // Here we are creating Vec<Box<dyn Any>> with different types of data
    let values: Vec<Box<dyn Any>> = vec![
        Box::new(10),
        Box::new(20),
        Box::new("String is not an integer"),
        Box::new(30),
    ];
    
    // Filtering only integers from the above values
    let integers = filter_integers(values);

    // Here only integers will be printed: [10, 20, 30]
    println!("{:?}", integers);
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
