fn main() {}

/*
Given a string representing a space separated lowercase letters, return a dictionary
    of the letter with the most repetition and containing the corresponding count.
    If several letters have the same occurrence, return all of them.

*/

use std::collections::HashMap;

fn histogram(test: &str) -> HashMap<char, i32> {
    let mut res: HashMap<char, i32> = HashMap::new();
    if test == "" {
        return res;
    }
    for c in test.split_ascii_whitespace() {
        if res.contains_key(&c.chars().next().unwrap()) {
            res.entry(c.chars().next().unwrap()).and_modify(|n| {
                *n += 1;
            });
        } else {
            res.insert(c.chars().next().unwrap(), 1);
        }
    }
    let max: i32 = *res.values().max().unwrap();
    let non_maxs: Vec<char> = res
        .keys()
        .filter(|k: &&char| *res.get(k).unwrap() != max)
        .map(|c| *c)
        .collect();
    non_maxs.iter().for_each(|c| {
        res.remove(c);
    });

    return res;
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
