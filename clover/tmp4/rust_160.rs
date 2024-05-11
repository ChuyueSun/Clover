
use std::collections::VecDeque;

fn do_algebra(operators: Vec<&str>, operands: Vec<i32>) -> i32 {
    let mut nums = VecDeque::from(operands);
    let mut result = nums.pop_front().unwrap();

    for op in operators {
        let right = nums.pop_front().unwrap();
        result = match op {
            "+" => result + right,
            "-" => result - right,
            "*" => result * right,
            "//" => result / right,
            "**" => result.pow(right as u32),
            _ => panic!("Invalid operator"),
        };
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do_algebra() {
        assert_eq!(do_algebra(vec!["+", "-", "*"], vec![2, 3, 1, 4]), 16);
        assert_eq!(do_algebra(vec!["//", "+", "**"], vec![32, 2, 3, 2]), 25);
        assert_eq!(do_algebra(vec!["**", "*", "//"], vec![2, 3, 2, 4]), 1);
        assert_eq!(do_algebra(vec!["*", "//", "+"], vec![1, 2, 2, 3]), 3);
    }
}

fn main() {
    // Example usage:
    let result = do_algebra(vec!["+", "-", "*"], vec![2, 3, 1, 4]);
    println!("The result of the algebraic expression is: {}", result);
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
