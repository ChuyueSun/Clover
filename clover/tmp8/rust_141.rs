
fn file_name_check(file_name: &str) -> &'static str {
    // Check for exactly one dot '.'
    let parts: Vec<&str> = file_name.split('.').collect();
    if parts.len() != 2 {
        return "No";
    }

    // Check the first part starts with a letter and doesn't exceed three digits,
    // The second part must be one of the specified extensions
    let (name, extension) = (parts[0], parts[1]);
    let is_valid_name = name.chars().next().map_or(false, |c| c.is_ascii_alphabetic())
        && name.chars().filter(|c| c.is_digit(10)).count() <= 3;
    let valid_extensions = ["txt", "exe", "dll"];
    let is_valid_extension = valid_extensions.contains(&extension);

    if is_valid_name && is_valid_extension {
        "Yes"
    } else {
        "No"
    }
}

fn main() {
    // Example usage:
    println!("{}", file_name_check("example.txt")); // Should print 'Yes'
    println!("{}", file_name_check("example2.dll")); // Should print 'Yes'
    println!("{}", file_name_check("2example.txt")); // Should print 'No'
    println!("{}", file_name_check("example.invalid")); // Should print 'No'
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
