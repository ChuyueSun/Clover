
fn closest_integer(value: &str) -> i32 {
    let num: f64 = value.parse().unwrap_or(0.0);
    if num.fract() == 0.5 || num.fract() == -0.5 {
        num.signum() * (num.abs() + 0.5).floor()
    } else { 
        num.round()
    } as i32
}

fn main() {
    println!("{}", closest_integer("14.5")); // should print 15
    println!("{}", closest_integer("-14.5")); // should print -15
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_closest_integer() {
        assert!(closest_integer("10") == 10);
        assert!(closest_integer("14.5") == 15);
        assert!(closest_integer("-15.5") == -16);
        assert!(closest_integer("15.3") == 15);
        assert!(closest_integer("0") == 0);
    }

}
