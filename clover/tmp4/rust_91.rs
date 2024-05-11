
fn is_bored(s: &str) -> usize {
    s.split(|c: char| ".?!".contains(c))
        .filter(|sentence| sentence.trim_start().starts_with("I "))
        .count()
}

fn main() {
    // Example usage:
    let example_text = "I am feeling bored today. Are you? I certainly am!";
    let boredom_count = is_bored(example_text);
    println!("The number of boredoms: {}", boredom_count);
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
