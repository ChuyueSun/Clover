fn main() {}

/*
You are given a string s.
    Your task is to check if the string is happy or not.
    A string is happy if its length is at least 3 and every 3 consecutive letters are distinct

*/

fn is_happy(s: &str) -> bool {
    let str: Vec<char> = s.chars().into_iter().collect();
    if str.len() < 3 {
        return false;
    }
    for i in 2..str.len() {
        if str[i] == str[i - 1] || str[i] == str[i - 2] {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_happy() {
        assert!(is_happy("a") == false);
        assert!(is_happy("aa") == false);
        assert!(is_happy("abcd") == true);
        assert!(is_happy("aabb") == false);
        assert!(is_happy("adb") == true);
        assert!(is_happy("xyy") == false);
        assert!(is_happy("iopaxpoi") == true);
        assert!(is_happy("iopaxioi") == false);
    }
}
