
use std::collections::HashMap;

fn histogram(test: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    let mut max_count = 0;

    // Count occurrences of each character
    for ch in test.chars().filter(|c| !c.is_whitespace()) {
        let count = counts.entry(ch).or_insert(0);
        *count += 1;
        max_count = max_count.max(*count);
    }

    // Filter the hashmap to only include characters with the maximum count
    counts.retain(|_, &mut count| count == max_count);

    counts
}

fn main() {
    let input = "a b c a b b c a a a b c c c c";
    let histogram = histogram(input);
    println!("{:?}", histogram);
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