fn main() {}

/*
 Out of list of strings, return the longest one. Return the first one in case of multiple
    strings of the same length. Return None in case the input list is empty.

*/

fn longest(strings: Vec<String>) -> Option<String> {
    if strings.is_empty() {
        return None;
    }
    let mut max: i32 = 0;
    let mut res: String = String::new();

    for s in strings {
        if s.len() as i32 > max {
            res = s;
            max = res.len() as i32;
        }
    }
    return Some(res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert!(longest(vec![]) == None);
        assert!(
            longest(vec!["x".to_string(), "y".to_string(), "z".to_string()])
                == Some("x".to_string())
        );
        assert!(
            longest(vec![
                "x".to_string(),
                "yyy".to_string(),
                "zzzz".to_string(),
                "www".to_string(),
                "kkkk".to_string(),
                "abc".to_string()
            ]) == Some("zzzz".to_string())
        );
    }
}