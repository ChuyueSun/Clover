
fn int_to_mini_roman(num: i32) -> String {
    let mut num = num;
    let mut roman = String::new();
    let numerals = [
        (1000, "m"), (900, "cm"), (500, "d"), (400, "cd"),
        (100, "c"), (90, "xc"), (50, "l"), (40, "xl"),
        (10, "x"), (9, "ix"), (5, "v"), (4, "iv"), (1, "i"),
    ];

    for &(value, symbol) in numerals.iter() {
        while num >= value {
            roman.push_str(symbol);
            num -= value;
        }
    }
    roman
}

fn main() {
    // Example use of `int_to_mini_roman`
    let number = 1990;
    let roman = int_to_mini_roman(number);
    println!("{}", roman); // Output: "mcmxc"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_to_mini_romank() {
        assert_eq!(int_to_mini_romank(19), "xix");
        assert_eq!(int_to_mini_romank(152), "clii");
        assert_eq!(int_to_mini_romank(251), "ccli");
        assert_eq!(int_to_mini_romank(426), "cdxxvi");
        assert_eq!(int_to_mini_romank(500), "d");
        assert_eq!(int_to_mini_romank(1), "i");
        assert_eq!(int_to_mini_romank(4), "iv");
        assert_eq!(int_to_mini_romank(43), "xliii");
        assert_eq!(int_to_mini_romank(90), "xc");
        assert_eq!(int_to_mini_romank(94), "xciv");
        assert_eq!(int_to_mini_romank(532), "dxxxii");
        assert_eq!(int_to_mini_romank(900), "cm");
        assert_eq!(int_to_mini_romank(994), "cmxciv");
        assert_eq!(int_to_mini_romank(1000), "m");
    }

}
