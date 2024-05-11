
fn bf(planet1: &str, planet2: &str) -> Vec<&'static str> {
    let planets = [
        "Mercury", "Venus", "Earth", "Mars", "Jupiter", "Saturn", "Uranus", "Neptune",
    ];

    let pos1 = planets.iter().position(|&p| p == planet1);
    let pos2 = planets.iter().position(|&p| p == planet2);

    if let (Some(p1), Some(p2)) = (pos1, pos2) {
        if p1 < p2 {
            return planets[p1 + 1..p2].to_vec();
        } else {
            return planets[p2 + 1..p1].to_vec();
        }
    }

    Vec::new()
}

fn main() {
    // Example usage:
    // This should print the planets between Earth and Saturn
    println!("{:?}", bf("Earth", "Saturn")); // ["Mars", "Jupiter"]
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
