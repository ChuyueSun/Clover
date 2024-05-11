
fn is_bored(s: &str) -> usize {
    let boredom_delimiters = ['.', '?', '!'];
    s.split(&boredom_delimiters[..])
        .map(str::trim)
        .filter(|sen| sen.starts_with('I') || sen.starts_with("I'm"))
        .count()
}

fn main() {
    let text = "I am bored. I'm really bored! Are you bored? I guess everyone is.";
    let boredom_count = is_bored(text);
    println!("Number of boredoms: {}", boredom_count);
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
