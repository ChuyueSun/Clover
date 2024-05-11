
fn do_algebra(operator: Vec<&str>, operand: Vec<i32>) -> i32 {
    let mut iter = operand.into_iter();
    let mut result = iter.next().unwrap(); // safe to unwrap because there is at least two operands
    
    for op in operator {
        // The unwrap here is safe because the length of operator is one less than the length of operand
        let next_num = iter.next().unwrap(); 
        result = match op {
            "+" => result + next_num,
            "-" => result - next_num,
            "*" => result * next_num,
            "**" => result.pow(next_num as u32),
            "//" => result / next_num, // Floor division is the default in Rust for integers
            _ => panic!("Unknown operator"), // If there's an unknown operator, we panic
        };
    }

    result
}

fn main() {
    let operators = vec!["+", "-", "*", "//", "**"];
    let operands = vec![2, 3, 4, 5, 6, 2];
    let result = do_algebra(operators, operands);
    println!("The result of the algebra operation is: {}", result);
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
