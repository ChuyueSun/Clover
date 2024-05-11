
fn fix_spaces(text: &str) -> String {
    let mut result = String::new();
    let mut space_count = 0;

    for c in text.chars() {
        if c == ' ' {
            space_count += 1;
            if space_count <= 2 {
                result.push('_');
            }
        } else {
            if space_count > 2 {
                if !result.ends_with('-') {
                    // Remove the underscores added for the first two spaces.
                    result.truncate(result.len() - space_count.min(2));
                    result.push('-');
                }
            }
            space_count = 0;
            result.push(c);
        }
    }

    if space_count > 2 && !result.ends_with('-') {
        // Remove the underscores added for the first two spaces and add the dash at the end.
        result.truncate(result.len() - space_count.min(2));
        result.push('-');
    }

    result
}

fn main() {
    // Example usage:
    let text = "This  is  some  text  with   multiple    spaces.";
    println!("{}", fix_spaces(text));
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
