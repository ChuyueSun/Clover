fn main() {}

/*
Change numerical base of input number x to base.
    return string representation after the conversion.
    base numbers are less than 10.

*/

fn change_base(x: i32, base: i32) -> String {
    let mut ret: String = "".to_string();
    let mut x1 = x;

    while x1 > 0 {
        ret = (x1 % base).to_string() + &ret;
        x1 = x1 / base;
    }
    return ret;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_base() {
        assert!(change_base(8, 3) == "22".to_string());
        assert!(change_base(9, 3) == "100".to_string());
        assert!(change_base(234, 2) == "11101010".to_string());
        assert!(change_base(16, 2) == "10000".to_string());
        assert!(change_base(8, 2) == "1000".to_string());
        assert!(change_base(7, 2) == "111".to_string());
    }
}
