fn main() {}

/*

    There are eight planets in our solar system: the closerst to the Sun
    is Mercury, the next one is Venus, then Earth, Mars, Jupiter, Saturn,
    Uranus, Neptune.
    Write a function that takes two planet names as strings planet1 and planet2.
    The function should return a tuple containing all planets whose orbits are
    located between the orbit of planet1 and the orbit of planet2, sorted by
    the proximity to the sun.
    The function should return an empty tuple if planet1 or planet2
    are not correct planet names.

*/

fn bf(planet1: &str, planet2: &str) -> Vec<String> {
    let planets = vec![
        "Mercury", "Venus", "Earth", "Mars", "Jupiter", "Saturn", "Uranus", "Neptune",
    ];
    let mut pos1: i32 = -1;
    let mut pos2: i32 = -1;
    let m;
    for m in 0..planets.len() {
        if planets[m] == planet1 {
            pos1 = m as i32;
        }
        if planets[m] == planet2 {
            pos2 = m as i32;
        }
    }
    if pos1 == -1 || pos2 == -1 {
        return vec![];
    }
    if pos1 > pos2 {
        m = pos1;
        pos1 = pos2;
        pos2 = m;
    }
    let mut out = vec![];
    for m in pos1 + 1..pos2 {
        out.push(planets[m as usize].to_string());
    }
    return out;
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
