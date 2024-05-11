
use std::collections::HashMap;

fn histogram(text: &str) -> HashMap<char, i32> {
    let mut frequency_map: HashMap<char, i32> = HashMap::new();
    let mut max_count = 0;

    // Count the frequency of each character
    for ch in text.chars().filter(|c| c.is_alphabetic()) {
        let count = frequency_map.entry(ch).or_insert(0);
        *count += 1;
        max_count = max_count.max(*count);
    }

    // Retain only the characters with the max frequency
    frequency_map.retain(|_, &mut count| count == max_count);
    frequency_map
}

fn main() {
    let sample_text = "a b c a b a";
    let histogram_map = histogram(sample_text);
    for (letter, count) in histogram_map {
        println!("{}: {}", letter, count);
    }
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
