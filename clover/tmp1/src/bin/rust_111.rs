
use std::collections::HashMap;

fn histogram(test: &str) -> HashMap<char, u32> {
    let mut freq_map: HashMap<char, u32> = HashMap::new();
    let mut max_freq = 0;

    // Count the frequency of each letter
    for c in test.split_whitespace().collect::<String>().chars() {
        let counter = freq_map.entry(c).or_insert(0);
        *counter += 1;
        max_freq = max_freq.max(*counter);
    }

    // Keep only the letters with the maximum frequency
    freq_map.retain(|_, &mut v| v == max_freq);

    freq_map
}

fn main() {
    // Example use
    let result = histogram("a a a b b c c c");
    println!("{:?}", result); // Output should be something like: {'a': 3, 'c': 3}
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
