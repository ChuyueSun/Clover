
fn file_name_check(file_name: &str) -> &'static str {
    // Check for a single dot and split at that position
    let parts: Vec<&str> = file_name.split('.').collect();
    if parts.len() != 2 {
        return "No";
    }
    let (name, extension) = (parts[0], parts[1]);

    // Check if the substring before the dot starts with a letter and is not empty
    if name.is_empty() || !name.chars().next().unwrap().is_ascii_alphabetic() {
        return "No";
    }

    // Check for the allowed number of digits in the substring before the dot
    let digit_count = name.chars().filter(|c| c.is_digit(10)).count();
    if digit_count > 3 {
        return "No";
    }

    // Check if the substring after the dot is one of the allowed extensions
    if !["txt", "exe", "dll"].contains(&extension) {
        return "No";
    }

    "Yes"
}

fn main() {
    // Example usage:
    // let valid = file_name_check("file123.txt");
    // println!("{}", valid); // Prints "Yes"
    
    // let invalid = file_name_check("file1234.txt");
    // println!("{}", invalid); // Prints "No"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_name_check() {
        assert_eq!(file_name_check("example.txt"), "Yes");
        assert_eq!(file_name_check("1example.dll"), "No");
        assert_eq!(file_name_check("s1sdf3.asd"), "No");
        assert_eq!(file_name_check("K.dll"), "Yes");
        assert_eq!(file_name_check("MY16FILE3.exe"), "Yes");
        assert_eq!(file_name_check("His12FILE94.exe"), "No");
        assert_eq!(file_name_check("_Y.txt"), "No");
        assert_eq!(file_name_check("?aREYA.exe"), "No");
        assert_eq!(file_name_check("/this_is_valid.dll"), "No");
        assert_eq!(file_name_check("this_is_valid.wow"), "No");
        assert_eq!(file_name_check("this_is_valid.txt"), "Yes");
        assert_eq!(file_name_check("this_is_valid.txtexe"), "No");
        assert_eq!(file_name_check("#this2_i4s_5valid.ten"), "No");
        assert_eq!(file_name_check("@this1_is6_valid.exe"), "No");
        assert_eq!(file_name_check("this_is_12valid.6exe4.txt"), "No");
        assert_eq!(file_name_check("all.exe.txt"), "No");
        assert_eq!(file_name_check("I563_No.exe"), "Yes");
        assert_eq!(file_name_check("Is3youfault.txt"), "Yes");
        assert_eq!(file_name_check("no_one#knows.dll"), "Yes");
        assert_eq!(file_name_check("1I563_Yes3.exe"), "No");
        assert_eq!(file_name_check("I563_Yes3.txtt"), "No");
        assert_eq!(file_name_check("final..txt"), "No");
        assert_eq!(file_name_check("final132"), "No");
        assert_eq!(file_name_check("_f4indsartal132."), "No");
        assert_eq!(file_name_check(".txt"), "No");
        assert_eq!(file_name_check("s."), "No");
    }

}
