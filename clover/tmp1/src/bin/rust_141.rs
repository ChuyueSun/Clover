
fn file_name_check(file_name: &str) -> &'static str {
    let parts: Vec<&str> = file_name.split('.').collect();
    if parts.len() != 2 {
        return "No";
    }

    let (name_part, extension_part) = (parts[0], parts[1]);
    let allowed_extensions = ["txt", "exe", "dll"];
    
    let is_valid_name = !name_part.is_empty() && 
                        name_part.chars().next().unwrap().is_alphabetic() &&
                        name_part.chars().filter(|c| c.is_digit(10)).count() <= 3;
    
    let is_valid_extension = allowed_extensions.contains(&extension_part);

    if is_valid_name && is_valid_extension {
        "Yes"
    } else {
        "No"
    }
}

fn main() {
    // Example usage:
    println!("{}", file_name_check("example.txt")); // Should print "Yes"
    println!("{}", file_name_check("badexample.txx")); // Should print "No"
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
