
fn file_name_check(file_name: &str) -> &'static str {
    let mut dot_count = 0;
    let mut digit_count = 0;
    let mut name = "";
    let valid_extensions = ["txt", "exe", "dll"];

    for (i, c) in file_name.char_indices() {
        if c.is_digit(10) {
            digit_count += 1;
        } else if c == '.' {
            dot_count += 1;
            if i == 0 || dot_count > 1 || digit_count > 3 {
                return "No";
            }
            name = &file_name[0..i];
            if !name.chars().next().unwrap().is_alphabetic() {
                return "No";
            }
        }
    }

    if dot_count != 1 || digit_count > 3 || !valid_extensions.contains(&file_name[name.len() + 1..]) {
        return "No";
    }

    "Yes"
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
