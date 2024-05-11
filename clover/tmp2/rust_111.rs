
use std::collections::HashMap;

fn histogram(test: &str) -> HashMap<char, usize> {
    let mut occurrences = HashMap::new();
    let mut max_count = 0;
    let mut histogram = HashMap::new();

    // Count occurrences of each character
    for c in test.chars().filter(|c| c.is_whitespace() == false) {
        let count = occurrences.entry(c).or_insert(0);
        *count += 1;
        max_count = max_count.max(*count);
    }

    // Select characters with the highest count
    for (character, &count) in &occurrences {
        if count == max_count {
            histogram.insert(*character, count);
        }
    }

    histogram
}

fn main() {
    // Example usage:
    let test_str = "a b b c c c";
    let histo = histogram(test_str);
    println!("{:?}", histo);
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
