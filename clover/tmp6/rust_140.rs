
fn fix_spaces(text: &str) -> String {
    let mut result = String::new();
    let mut space_count = 0;

    for char in text.chars() {
        if char == ' ' {
            space_count += 1;
            if space_count == 3 {
                result.pop();
                result.pop();
                result.push('-');
            } else if space_count > 3 {
                continue;
            } else {
                result.push('_');
            }
        } else {
            space_count = 0;
            result.push(char);
        }
    }
    result
}

fn main() {
    // Example usage:
    let text = "This  is  a   test    string";
    println!("{}", fix_spaces(text)); // Output: This__is__a-test-string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_spaces() {
        assert_eq!(fix_spaces("Example"), "Example");
        assert_eq!(fix_spaces("Mudasir Hanif "), "Mudasir_Hanif_");
        assert_eq!(
            fix_spaces("Yellow Yellow  Dirty  Fellow"),
            "Yellow_Yellow__Dirty__Fellow"
        );
        assert_eq!(fix_spaces("Exa   mple"), "Exa-mple");
        assert_eq!(fix_spaces("   Exa 1 2 2 mple"), "-Exa_1_2_2_mple");
    }

}
