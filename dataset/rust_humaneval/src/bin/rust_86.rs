fn main() {}

/*

    Write a function that takes a string and returns an ordered version of it.
    Ordered version of string, is a string where all words (separated by space)
    are replaced by a new word where all the characters arranged in
    ascending order based on ascii value.
    Note: You should keep the order of words and blank spaces in the sentence.

*/

fn anti_shuffle(s: &str) -> String {
    let mut res: String = String::new();

    for i in s.split_ascii_whitespace() {
        let mut str: Vec<char> = i.chars().into_iter().collect();
        str.sort_by(|a, b| (*a as u32).cmp(&(*b as u32)));
        let str_sorted: String = str.into_iter().collect();
        res.push_str(&(str_sorted + &" ".to_string()));
    }
    res = res.trim_end().to_string();
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anti_shuffle() {
        assert!(anti_shuffle("Hi") == "Hi".to_string());
        assert!(anti_shuffle("hello") == "ehllo".to_string());
        assert!(anti_shuffle("number") == "bemnru".to_string());
        assert!(anti_shuffle("abcd") == "abcd".to_string());
        assert!(anti_shuffle("Hello World!!!") == "Hello !!!Wdlor".to_string());
        assert!(anti_shuffle("") == "".to_string());
        assert!(
            anti_shuffle("Hi. My name is Mister Robot. How are you?")
                == ".Hi My aemn is Meirst .Rboot How aer ?ouy".to_string()
        );
    }
}
