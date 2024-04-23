fn main() {}

/*

    In this Kata, you have to sort an array of non-negative integers according to
    number of ones in their binary representation in ascending order.
    For similar number of ones, sort based on decimal value.

*/

fn sort_array_1(arr: Vec<i32>) -> Vec<i32> {
    let mut arr_cp = arr.clone();
    let mut bin = vec![];
    let mut m;

    for i in 0..arr_cp.len() {
        let mut b = 0;
        let mut n = arr_cp[i].abs();
        while n > 0 {
            b += n % 2;
            n = n / 2;
        }
        bin.push(b);
    }
    for _i in 0..arr_cp.len() {
        for j in 1..arr_cp.len() {
            if bin[j] < bin[j - 1] || (bin[j] == bin[j - 1] && arr_cp[j] < arr_cp[j - 1]) {
                m = arr_cp[j];
                arr_cp[j] = arr_cp[j - 1];
                arr_cp[j - 1] = m;
                m = bin[j];
                bin[j] = bin[j - 1];
                bin[j - 1] = m;
            }
        }
    }
    return arr_cp;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_array_1() {
        assert!(sort_array_1(vec![1, 5, 2, 3, 4]) == vec![1, 2, 4, 3, 5]);
        assert!(sort_array_1(vec![-2, -3, -4, -5, -6]) == vec![-4, -2, -6, -5, -3]);
        assert!(sort_array_1(vec![1, 0, 2, 3, 4]) == vec![0, 1, 2, 4, 3]);
        assert!(sort_array_1(vec![]) == vec![]);
        assert!(
            sort_array_1(vec![2, 5, 77, 4, 5, 3, 5, 7, 2, 3, 4])
                == vec![2, 2, 4, 4, 3, 3, 5, 5, 5, 7, 77]
        );
        assert!(sort_array_1(vec![3, 6, 44, 12, 32, 5]) == vec![32, 3, 5, 6, 12, 44]);
        assert!(sort_array_1(vec![2, 4, 8, 16, 32]) == vec![2, 4, 8, 16, 32]);
        assert!(sort_array_1(vec![2, 4, 8, 16, 32]) == vec![2, 4, 8, 16, 32]);
    }
}
