
fn file_name_check(file_name: &str) -> &'static str {
    let parts: Vec<&str> = file_name.split('.').collect();

    // Check if the file name contains exactly one dot
    if parts.len() != 2 {
        return "No";
    }

    let (name, ext) = (parts[0], parts[1]);
    let extension_allowed = ["txt", "exe", "dll"];
    
    // Check if extension is valid
    if !extension_allowed.contains(&ext) {
        return "No";
    }

    // Check that the name starts with a letter and has at most three digits
    let mut digit_count = 0;
    for (i, c) in name.chars().enumerate() {
        if c.is_digit(10) {
            digit_count += 1;
        } else if i == 0 && !c.is_alphabetic() {
            return "No";
        }
    }

    if digit_count > 3 || name.is_empty() {
        return "No";
    }

    "Yes"
}

fn main() {
    // Test cases
    println!("{}", file_name_check("file123.txt")); // Yes
    println!("{}", file_name_check("123file.txt")); // No
    println!("{}", file_name_check("file.123.txt")); // No
    println!("{}", file_name_check("file123.exe")); // Yes
    println!("{}", file_name_check("file123.dll")); // Yes
    println!("{}", file_name_check(".file123.txt")); // No
    println!("{}", file_name_check("file123.tx")); // No
    println!("{}", file_name_check("file1234.txt")); // No
    println!("{}", file_name_check("")); // No
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
