fn main() {}

/*
 For a given list of integers, return a tuple consisting of a sum and a product of all the integers in a list.
    Empty sum should be equal to 0 and empty product should be equal to 1.

*/

fn sum_product(numbers: Vec<i32>) -> (i32, i32) {
    let sum = |xs: &Vec<i32>| {
        xs.iter().fold(0, |mut sum, &val| {
            sum += val;
            sum
        })
    };
    let product = |xs: &Vec<i32>| {
        xs.iter().fold(1, |mut prod, &val| {
            prod *= val;
            prod
        })
    };
    return (sum(&numbers), product(&numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_product() {
        assert!(sum_product(vec![]) == (0, 1));
        assert!(sum_product(vec![1, 1, 1]) == (3, 1));
        assert!(sum_product(vec![100, 0]) == (100, 0));
        assert!(sum_product(vec![3, 5, 7]) == (3 + 5 + 7, 3 * 5 * 7));
        assert!(sum_product(vec![10]) == (10, 10));
    }
}
