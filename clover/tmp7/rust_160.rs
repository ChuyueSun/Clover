
fn do_algebra(operato: Vec<&str>, operand: Vec<i32>) -> i32 {
    if operato.len() != operand.len() - 1 {
        panic!("The number of operators must be one less than the number of operands.");
    }

    let mut result = operand[0];
    for (i, op) in operato.iter().enumerate() {
        match *op {
            "+" => result += operand[i + 1],
            "-" => result -= operand[i + 1],
            "*" => result *= operand[i + 1],
            "//" => result /= operand[i + 1],
            "**" => result = result.pow(operand[i + 1] as u32),
            _ => panic!("Unknown operator: {}", op),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do_algebra() {
        assert_eq!(do_algebra(vec!["**", "*", "+"], vec![2, 3, 4, 5]), 37);
        assert_eq!(do_algebra(vec!["+", "*", "-"], vec![2, 3, 4, 5]), 9);
        assert_eq!(do_algebra(vec!["//", "*"], vec![7, 3, 4]), 8);
    }


}
