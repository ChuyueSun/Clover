
fn bf(planet1: &str, planet2: &str) -> (&str, Vec<&str>, &str) {
    let planets = [
        "Mercury", "Venus", "Earth", "Mars", "Jupiter", "Saturn", "Uranus", "Neptune",
    ];
    
    let pos1 = planets.iter().position(|&p| p.eq(planet1));
    let pos2 = planets.iter().position(|&p| p.eq(planet2));
    
    match (pos1, pos2) {
        (Some(start), Some(end)) if start < end => (
            planet1,
            planets[start + 1..end].to_vec(),
            planet2,
        ),
        (Some(start), Some(end)) => (
            planet2,
            planets[end + 1..start].to_vec(),
            planet1,
        ),
        _ => {
            return ("", vec![], "");
        }
    }
}

fn main() {
    // Example usage:
    let result = bf("Mercury", "Mars");
    println!("{:?}", result); // Expected output: ("Mercury", ["Venus", "Earth"], "Mars")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bf() {
        assert_eq!(bf("Jupiter", "Neptune"), vec!["Saturn", "Uranus"]);
        assert_eq!(bf("Earth", "Mercury"), vec!["Venus"]);
        assert_eq!(
            bf("Mercury", "Uranus"),
            vec!["Venus", "Earth", "Mars", "Jupiter", "Saturn"]
        );
        assert_eq!(
            bf("Neptune", "Venus"),
            vec!["Earth", "Mars", "Jupiter", "Saturn", "Uranus"]
        );
        let v_empty: Vec<&str> = vec![];
        assert_eq!(bf("Earth", "Earth"), v_empty);
        assert_eq!(bf("Mars", "Earth"), v_empty);
        assert_eq!(bf("Jupiter", "Makemake"), v_empty);
    }

}
