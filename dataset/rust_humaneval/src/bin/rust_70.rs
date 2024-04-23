fn main() {}

/*

    Given list of integers, return list in strange order.
    Strange sorting, is when you start with the minimum value,
    then maximum of the remaining integers, then minimum and so on.

*/

fn strange_sort_list(lst: Vec<i32>) -> Vec<i32> {
    let mut cp: Vec<i32> = lst.clone();
    let mut res: Vec<i32> = vec![];

    for (indx, _) in lst.iter().enumerate() {
        if indx % 2 == 1 {
            let max: i32 = *cp.iter().max().unwrap();
            res.push(max);
            cp.remove(cp.iter().position(|x| *x == max).unwrap());
        } else {
            let min: i32 = *cp.iter().min().unwrap();
            res.push(min);
            cp.remove(cp.iter().position(|x| *x == min).unwrap());
        }
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strange_sort_list() {
        assert!(strange_sort_list(vec![1, 2, 3, 4]) == vec![1, 4, 2, 3]);
        assert!(strange_sort_list(vec![5, 6, 7, 8, 9]) == vec![5, 9, 6, 8, 7]);
        assert!(strange_sort_list(vec![1, 2, 3, 4, 5]) == vec![1, 5, 2, 4, 3]);
        assert!(strange_sort_list(vec![5, 6, 7, 8, 9, 1]) == vec![1, 9, 5, 8, 6, 7]);
        assert!(strange_sort_list(vec![5, 5, 5, 5]) == vec![5, 5, 5, 5]);
        assert!(strange_sort_list(vec![]) == vec![]);
        assert!(strange_sort_list(vec![1, 2, 3, 4, 5, 6, 7, 8]) == vec![1, 8, 2, 7, 3, 6, 4, 5]);
        assert!(
            strange_sort_list(vec![0, 2, 2, 2, 5, 5, -5, -5]) == vec![-5, 5, -5, 5, 0, 2, 2, 2]
        );
        assert!(strange_sort_list(vec![111111]) == vec![111111]);
    }
}
