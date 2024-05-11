
fn do_algebra(operators: Vec<&str>, operands: Vec<i32>) -> i32 {
    use std::collections::VecDeque;

    let mut nums = VecDeque::from(operands);
    let mut ops = operators.iter();

    let mut result = nums.pop_front().expect("There should be at least one operand");

    while let Some(&op) = ops.next() {
        let next_num = nums.pop_front().expect("There should be enough operands for operators");

        result = match op {
            "+" => result + next_num,
            "-" => result - next_num,
            "*" => result * next_num,
            "//" => result / next_num,
            "**" => result.pow(next_num as u32),
            _ => unreachable!("Unsupported operator"),
        };
    }

    result
}

fn main() {
    // Example usage:
    let operators = vec!["+", "-", "*", "//", "**"];
    let operands = vec![1, 2, 3, 4, 5, 6];
    let result = do_algebra(operators, operands);
    println!("Result: {}", result);
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
