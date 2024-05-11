
use std::collections::HashMap;

fn histogram(test: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    let mut max_count = 0;
    
    // Count the occurrences of each letter
    for c in test.chars().filter(|c| c.is_alphabetic()) {
        let count = counts.entry(c).or_insert(0);
        *count += 1;
        max_count = max_count.max(*count);
    }

    // Retain only the letters with the maximum occurrence
    counts.retain(|_, &mut v| v == max_count);
    counts
}

fn main() {
    let test_str = "a b c a a d d d d b b b";
    let result = histogram(test_str);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_histogram() {
        assert!(histogram("a b b a") == HashMap::from([('a', 2), ('b', 2)]));
        assert!(histogram("a b c a b") == HashMap::from([('a', 2), ('b', 2)]));
        assert!(
            histogram("a b c d g")
                == HashMap::from([('a', 1), ('b', 1), ('c', 1), ('d', 1), ('g', 1)])
        );
        assert!(histogram("r t g") == HashMap::from([('r', 1), ('t', 1), ('g', 1)]));
        assert!(histogram("b b b b a") == HashMap::from([('b', 4)]));
        assert!(histogram("r t g") == HashMap::from([('r', 1), ('t', 1), ('g', 1)]));
        assert!(histogram("") == HashMap::new());
        assert!(histogram("a") == HashMap::from([(('a', 1))]));
    }

}
