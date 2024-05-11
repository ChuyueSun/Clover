
use std::collections::HashMap;

fn histogram(test: &str) -> HashMap<char, i32> {
    let mut frequencies = HashMap::new();
    let mut max_freq = 0;

    // Calculate the frequency of each letter
    for c in test.chars().filter(|c| c.is_alphabetic()) {
        let counter = frequencies.entry(c).or_insert(0);
        *counter += 1;
        max_freq = max_freq.max(*counter);
    }

    // Only retain letters that have the highest frequency
    frequencies.retain(|_, &mut v| v == max_freq);
    frequencies
}

fn main() {
    let test = "a b c a b a";
    let histogram = histogram(test);
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
