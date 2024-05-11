
fn do_algebra(operators: Vec<&str>, operands: Vec<i32>) -> i32 {
    let mut iter = operands.iter().cloned();
    let mut result = iter.next().unwrap();
    
    for (op, &num) in operators.iter().zip(iter) {
        result = match op {
            "+" => result + num,
            "-" => result - num,
            "*" => result * num,
            "//" => result / num,
            "**" => result.pow(num as u32),
            _ => unreachable!("Unsupported operator."),
        };
    }

    result
}

fn main() {
    // Example usage:
    let operators = vec!["+", "*", "-", "//"];
    let operands = vec![3, 5, 2, 4, 2];
    println!("{}", do_algebra(operators, operands)); // Should output the result of the expression
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
