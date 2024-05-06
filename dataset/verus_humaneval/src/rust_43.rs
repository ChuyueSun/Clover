use vstd::prelude::*;
fn main() {}

/*
    pairs_sum_to_zero takes a list of integers as an input.
    it returns True if there are two distinct elements in the list that
    sum to zero, and False otherwise.
*/

verus! {
fn pairs_sum_to_zero(l: Vec<i32>) -> (r: bool)
    requires
        forall|i: int| 0 <= i < l.len() ==> -0x1000_0000 <= #[trigger] l[i] <= 0x1000_0000,
    ensures
        r == (exists |i: int, j: int| 0 <= i < j < l.len() && l[i as int] + l[j as int] == 0)
{
    for i in 0..l.len()
        invariant
            0 <= i <= l.len(),
            forall|i_: int, j_: int| 0 <= i_ < i && i_ < j_ < l.len() ==> l[i_ as int] + l[j_ as int] != 0,
            // overflow check
            forall|i: int| 0 <= i < l.len() ==> -0x1000_0000 <= #[trigger] l[i] <= 0x1000_0000,
    {
        for j in i + 1..l.len()
            invariant
                i < j <= l.len(),
                forall|j_: int| i < j_ < j ==> l[i as int] + l[j_ as int] != 0,
                // overflow check
                forall|i: int| 0 <= i < l.len() ==> -0x1000_0000 <= #[trigger] l[i] <= 0x1000_0000,
        {
            assert(-0x1000_0000 <= l[i as int] <= 0x1000_0000);
            assert(-0x1000_0000 <= l[j as int] <= 0x1000_0000);
            if l[i] + l[j] == 0 {
                return true;
            }
        }
    }

    return false;
}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pairs_sum_to_zero() {
        assert!(pairs_sum_to_zero(vec![1, 3, 5, 0]) == false);
        assert!(pairs_sum_to_zero(vec![1, 3, -2, 1]) == false);
        assert!(pairs_sum_to_zero(vec![1, 2, 3, 7]) == false);
        assert!(pairs_sum_to_zero(vec![2, 4, -5, 3, 5, 7]) == true);
        assert!(pairs_sum_to_zero(vec![1]) == false);
        assert!(pairs_sum_to_zero(vec![-3, 9, -1, 3, 2, 30]) == true);
        assert!(pairs_sum_to_zero(vec![-3, 9, -1, 3, 2, 31]) == true);
        assert!(pairs_sum_to_zero(vec![-3, 9, -1, 4, 2, 30]) == false);
        assert!(pairs_sum_to_zero(vec![-3, 9, -1, 4, 2, 31]) == false);
    }
}
