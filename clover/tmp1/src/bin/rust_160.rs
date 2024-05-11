
fn do_algebra(operators: Vec<&str>, operands: Vec<i32>) -> i32 {
    let mut iter_operands = operands.into_iter();
    let mut result = iter_operands.next().unwrap();

    for (op, operand) in operators.into_iter().zip(iter_operands) {
        result = match op {
            "+" => result + operand,
            "-" => result - operand,
            "*" => result * operand,
            "//" => result / operand, // Assuming '//' means floor division in integer context
            "**" => result.pow(operand as u32), // Assuming '**' means exponentiation
            _ => panic!("Unsupported operator: {}", op), // panic for unsupported operators
        };
    }

    result
}

fn main() {
    // Example usage:
    println!("{}", do_algebra(vec!["+", "-", "*"], vec![2, 3, 5, 2])); // Example output: 3
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
