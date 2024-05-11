
fn order_by_points(mut arr: Vec<i32>) -> Vec<i32> {
    fn sum_of_digits(mut n: i32) -> i32 {
        let mut sum = 0;
        while n > 0 {
            sum += n % 10;
            n /= 10;
        }
        sum
    }

    arr.sort_by(|a, b| {
        let a_sum = sum_of_digits(*a);
        let b_sum = sum_of_digits(*b);
        a_sum.cmp(&b_sum).then_with(|| arr.iter().position(|x| x == a).cmp(&arr.iter().position(|x| x == b)))
    });

    arr
}

fn main() {
    let arr = vec![56, 81, 92, 43, 65, 31, 57];
    let sorted_arr = order_by_points(arr);
    println!("{:?}", sorted_arr);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_by_points() {
        assert_eq!(
            order_by_points(vec![1, 11, -1, -11, -12]),
            vec![-1, -11, 1, -12, 11]
        );
        assert_eq!(
            order_by_points(vec![
                1234, 423, 463, 145, 2, 423, 423, 53, 6, 37, 3457, 3, 56, 0, 46
            ]),
            vec![0, 2, 3, 6, 53, 423, 423, 423, 1234, 145, 37, 46, 56, 463, 3457]
        );
        assert_eq!(order_by_points(vec![]), vec![]);
        assert_eq!(
            order_by_points(vec![1, -11, -32, 43, 54, -98, 2, -3]),
            vec![-3, -32, -98, -11, 1, 2, 43, 54]
        );
        assert_eq!(
            order_by_points(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]),
            vec![1, 10, 2, 11, 3, 4, 5, 6, 7, 8, 9]
        );
        assert_eq!(
            order_by_points(vec![0, 6, 6, -76, -21, 23, 4]),
            vec![-76, -21, 0, 4, 23, 6, 6]
        );
    }

}
