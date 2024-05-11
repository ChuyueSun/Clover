
/// Gets the tuple of planets between two given planets.
///
/// Args:
///     planet1 (str): The name of the first planet.
///     planet2 (str): The name of the second planet.
///
/// Returns:
///     tuple: A tuple containing the names of the planets that are between the
///     two given planets, sorted by their distance from the Sun.
fn get_planets_between(planet1: &str, planet2: &str) -> Vec<&str> {
    let planets = [
        "Mercury", "Venus", "Earth", "Mars",
        "Jupiter", "Saturn", "Uranus", "Neptune"
    ];
    
    let (start_idx, end_idx) = match (planets.iter().position(|&p| p == planet1), planets.iter().position(|&p| p == planet2)) {
        (Some(start), Some(end)) => if start < end { (start, end) } else { (end, start) },
        _ => return Vec::new(),
    };
    
    planets[start_idx + 1..end_idx].to_vec()
}

fn bf(planet1: &str, planet2: &str) -> Vec<&str> {
    get_planets_between(planet1, planet2)
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
