
fn is_bored(s: &str) -> u32 {
    let mut boredom_count = 0;
    let sentences = s.split_terminator(|c| c == '.' || c == '?' || c == '!');
    for sentence in sentences {
        let trimmed_sentence = sentence.trim();
        if trimmed_sentence.starts_with("I") || trimmed_sentence.starts_with("I".to_lowercase().as_str()) {
            boredom_count += 1;
        }
    }
    boredom_count
}

fn main() {
    // Example usage:
    let text = "I am feeling bored. Are you bored? I wonder what to do!";
    let count = is_bored(text);
    println!("Number of boredoms: {}", count); // Should output: Number of boredoms: 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_bored() {
        assert!(is_bored("Hello world") == 0);
        assert!(is_bored("Is the sky blue?") == 0);
        assert!(is_bored("I love It !") == 1);
        assert!(is_bored("bIt") == 0);
        assert!(is_bored("I feel good today. I will be productive. will kill It") == 2);
        assert!(is_bored("You and I are going for a walk") == 0);
    }

}
