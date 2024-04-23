fn main() {}

/*
Task
    We are given two strings s and c, you have to deleted all the characters in s that are equal to any character in c
    then check if the result string is palindrome.
    A string is called palindrome if it reads the same backward as forward.
    You should return a tuple containing the result string and True/False for the check.

*/

fn reverse_delete(s: &str, c: &str) -> Vec<String> {
    let mut n = String::new();
    for i in 0..s.len() {
        if !c.contains(s.chars().nth(i).unwrap()) {
            n.push(s.chars().nth(i).unwrap());
        }
    }
    if n.len() == 0 {
        return vec![n, "True".to_string()];
    }
    let w: String = n.chars().rev().collect();
    if w == n {
        return vec![n, "True".to_string()];
    }
    return vec![n, "False".to_string()];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_delete() {
        assert!(reverse_delete("abcde", "ae") == ["bcd", "False"]);
        assert!(reverse_delete("abcdef", "b") == ["acdef", "False"]);
        assert!(reverse_delete("abcdedcba", "ab") == ["cdedc", "True"]);
        assert!(reverse_delete("dwik", "w") == ["dik", "False"]);
        assert!(reverse_delete("a", "a") == ["", "True"]);
        assert!(reverse_delete("abcdedcba", "") == ["abcdedcba", "True"]);
        assert!(reverse_delete("abcdedcba", "v") == ["abcdedcba", "True"]);
        assert!(reverse_delete("vabba", "v") == ["abba", "True"]);
        assert!(reverse_delete("mamma", "mia") == ["", "True"]);
    }
}
