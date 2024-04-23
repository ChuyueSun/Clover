fn main() {}

/*
 From a supplied list of numbers (of length at least two) select and return two that are the closest to each
    other and return them in order (smaller number, larger number).

*/

fn find_closest_elements(numbers: Vec<f32>) -> (f32, f32) {
    let mut closest_pair = (0.0, 0.0);
    let mut distance: Option<f32> = None;

    for (idx, elem) in numbers.iter().enumerate() {
        for (idx2, elem2) in numbers.iter().enumerate() {
            if idx != idx2 {
                if distance == None {
                    distance = Some((elem - elem2).abs());
                    if *elem < *elem2 {
                        closest_pair = (*elem, *elem2);
                    } else {
                        closest_pair = (*elem2, *elem);
                    }
                } else {
                    let new_distance: f32 = (elem - elem2).abs();
                    if new_distance < distance.unwrap() {
                        distance = Some(new_distance);

                        if *elem < *elem2 {
                            closest_pair = (*elem, *elem2);
                        } else {
                            closest_pair = (*elem2, *elem);
                        }
                    }
                }
            }
        }
    }
    return closest_pair;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_closest_elements() {
        assert!(find_closest_elements(vec![1.0, 2.0, 3.9, 4.0, 5.0, 2.2]) == (3.9, 4.0));
        assert!(find_closest_elements(vec![1.0, 2.0, 5.9, 4.0, 5.0]) == (5.0, 5.9));
        assert!(find_closest_elements(vec![1.0, 2.0, 3.0, 4.0, 5.0, 2.2]) == (2.0, 2.2));
        assert!(find_closest_elements(vec![1.0, 2.0, 3.0, 4.0, 5.0, 2.0]) == (2.0, 2.0));
        assert!(find_closest_elements(vec![1.1, 2.2, 3.1, 4.1, 5.1]) == (2.2, 3.1));
    }
}
