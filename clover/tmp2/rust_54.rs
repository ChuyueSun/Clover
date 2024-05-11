
fn same_chars(str1: &str, str2: &str) -> bool {
    let mut chars1: Vec<char> = str1.chars().collect();
    let mut chars2: Vec<char> = str2.chars().collect();
    chars1.sort_unstable();
    chars2.sort_unstable();
    chars1 == chars2
}

fn main() {
    let str1 = "listen";
    let str2 = "silent";
    println!("Do '{}' and '{}' have the same characters? {}", str1, str2, same_chars(str1, str2));
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_same_chars() {
        assert!(same_chars("eabcdzzzz", "dddzzzzzzzddeddabc") == true);
        assert!(same_chars("abcd", "dddddddabc") == true);
        assert!(same_chars("dddddddabc", "abcd") == true);
        assert!(same_chars("eabcd", "dddddddabc") == false);
        assert!(same_chars("abcd", "dddddddabcf") == false);
        assert!(same_chars("eabcdzzzz", "dddzzzzzzzddddabc") == false);
        assert!(same_chars("aabb", "aaccc") == false);
    }

}
