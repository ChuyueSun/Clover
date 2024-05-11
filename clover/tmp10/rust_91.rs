
fn is_bored(s: &str) -> usize {
    s.split_terminator(|c: char| ['.', '?', '!'].contains(&c))
        .map(str::trim)
        .filter(|sentence| sentence.starts_with("I ") || sentence.starts_with("I'"))
        .count()
}

fn main() {
    let text = "I am learning Rust. It is interesting! I'm hungry though.";
    let boredom_count = is_bored(text);
    println!("There are {} boredom sentences.", boredom_count);
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
