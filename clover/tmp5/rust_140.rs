
fn fix_spaces(text: &str) -> String {
    let mut result = String::new();
    let mut space_count = 0;

    for c in text.chars() {
        if c == ' ' {
            space_count += 1;
            if space_count == 2 {
                // Remove the previously added underscore to prepare for dashes
                result.pop();
                result.push('-');
            } else if space_count > 2 {
                // Do nothing, since we are replacing with dashes
                continue;
            } else {
                result.push('_');
            }
        } else {
            space_count = 0;
            result.push(c);
        }
    }

    result
}

fn main() {
    // Example usage:
    let text = "Hello   World! This is a test.";
    println!("Original: {}", text);
    println!("Fixed: {}", fix_spaces(text));
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
