
fn do_algebra(operators: Vec<&str>, operands: Vec<i32>) -> i32 {
    let mut result = operands[0];
    
    for (i, &op) in operators.iter().enumerate() {
        match op {
            "+" => result += operands[i + 1],
            "-" => result -= operands[i + 1],
            "*" => result *= operands[i + 1],
            "//" => result /= operands[i + 1],
            "**" => result = result.pow(operands[i + 1] as u32),
            _ => panic!("Invalid operator"),
        }
    }
    
    result
}

fn main() {
    // Example usage:
    let operators = vec!["+", "-", "*", "//", "**"];
    let operands = vec![1, 2, 3, 4, 5, 2]; // The length of `operands` is `operators` length + 1
    let result = do_algebra(operators, operands);
    println!("The result of the algebraic operations is: {}", result);
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