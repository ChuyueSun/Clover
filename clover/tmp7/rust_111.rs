
use std::collections::HashMap;

fn histogram(test: &str) -> HashMap<char, usize> {
    let mut letter_counts = HashMap::new();

    for c in test.chars().filter(|c| c.is_alphabetic()) {
        *letter_counts.entry(c).or_insert(0) += 1;
    }

    let max_count = letter_counts.values().max().cloned().unwrap_or(0);
    letter_counts.into_iter().filter(|&(_, v)| v == max_count).collect()
}

fn main() {
    let test_string = "some string with some letters";
    let hist = histogram(test_string);
    println!("{:?}", hist);
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
