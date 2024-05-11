
fn file_name_check(file_name: &str) -> &'static str {
    let parts: Vec<&str> = file_name.split('.').collect();
    let valid_extensions = ["txt", "exe", "dll"];

    if parts.len() == 2
        && parts[0].chars().next().map_or(false, |c| c.is_ascii_alphabetic())
        && parts[0].chars().filter(|c| c.is_digit(10)).count() <= 3
        && valid_extensions.contains(&parts[1])
    {
        "Yes"
    } else {
        "No"
    }
}

fn main() {
    // Example usage:
    let file_name1 = "data2.txt";
    let file_name2 = "data04.exe";
    let file_name3 = "2update3.dll";
    let file_name4 = "hello..txt";
    let file_name5 = "invalid_file.bmp";

    println!("Is '{}' valid? {}", file_name1, file_name_check(file_name1)); // Yes
    println!("Is '{}' valid? {}", file_name2, file_name_check(file_name2)); // Yes
    println!("Is '{}' valid? {}", file_name3, file_name_check(file_name3)); // No
    println!("Is '{}' valid? {}", file_name4, file_name_check(file_name4)); // No
    println!("Is '{}' valid? {}", file_name5, file_name_check(file_name5)); // No
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
