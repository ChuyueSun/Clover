
fn fix_spaces(text: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = text.chars().collect();
    let mut space_count = 0;

    for &ch in &chars {
        if ch == ' ' {
            space_count += 1;
        } else {
            if space_count > 2 {
                result.push('-');
            } else {
                result.extend(std::iter::repeat('_').take(space_count));
            }
            space_count = 0;
            result.push(ch);
        }
    }

    if space_count > 2 {
        result.push('-');
    } else {
        result.extend(std::iter::repeat('_').take(space_count));
    }

    result
}

fn main() {
    // Example usage
    let example_text = "This   is a     test";
    let fixed_text = fix_spaces(example_text);
    println!("{}", fixed_text); // Should output: "This-is_a_test"
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
