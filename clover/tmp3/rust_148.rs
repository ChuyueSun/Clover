
fn bf(planet1: &str, planet2: &str) -> Vec<&str> {
    let planets = vec![
        "Mercury", "Venus", "Earth", "Mars", "Jupiter", "Saturn", "Uranus", "Neptune",
    ];

    let position1 = planets.iter().position(|&p| p == planet1);
    let position2 = planets.iter().position(|&p| p == planet2);

    match (position1, position2) {
        (Some(pos1), Some(pos2)) => {
            if pos1 < pos2 {
                planets[pos1 + 1..pos2].to_vec()
            } else {
                planets[pos2 + 1..pos1].to_vec()
            }
        }
        _ => vec![],
    }
}

// Example usage:
// let planets_between = bf("Earth", "Jupiter");
// assert_eq!(planets_between, vec!["Mars", "Jupiter"]);

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
