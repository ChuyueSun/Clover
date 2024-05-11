
fn file_name_check(file_name: &str) -> &'static str {
    let parts: Vec<&str> = file_name.split('.').collect();
    if parts.len() != 2
        || parts[0].is_empty()
        || !parts[0].chars().next().unwrap().is_alphabetic()
        || parts[0].chars().filter(|c| c.is_digit(10)).count() > 3
        || !["txt", "exe", "dll"].contains(&parts[1])
    {
        "No"
    } else {
        "Yes"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_file_names() {
        assert_eq!(file_name_check("file123.txt"), "Yes");
        assert_eq!(file_name_check("a2.exe"), "Yes");
        assert_eq!(file_name_check("A9.dll"), "Yes");
    }

    #[test]
    fn test_invalid_file_names() {
        assert_eq!(file_name_check("file1234.txt"), "No"); // More than three digits
        assert_eq!(file_name_check("file.txt.exe"), "No"); // More than one dot
        assert_eq!(file_name_check(".txt"), "No"); // Empty substring before dot
        assert_eq!(file_name_check("3file.txt"), "No"); // Starts with digit
        assert_eq!(file_name_check("noextension"), "No"); // No dot at all
        assert_eq!(file_name_check("file.doc"), "No"); // Wrong extension
    }
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
