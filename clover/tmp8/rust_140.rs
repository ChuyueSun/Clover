
fn fix_spaces(text: &str) -> String {
    let mut result = String::new();
    let mut space_count = 0;

    for c in text.chars() {
        if c == ' ' {
            space_count += 1;
            if space_count == 3 {
                let len = result.len();
                result.truncate(len - 2); // remove the underscores added for the first two spaces
                result.push('-');
            } else if space_count > 3 {
                continue; // we have already replaced with a '-', just skip the extra space/s
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
    let text = "Here is   some text with   excessive  spaces";
    let fixed = fix_spaces(text);
    println!("{}", fixed);
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
