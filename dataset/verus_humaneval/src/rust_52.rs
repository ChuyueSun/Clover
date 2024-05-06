use vstd::prelude::*;
fn main() {}

/*
Return True if all numbers in the list l are below threshold t.
*/

verus! {
fn below_threshold(l: Vec<i32>, t: i32) -> (r: bool)
    ensures
        r == (forall|i: int| 0 <= i < l.len() ==> l[i] < t),
{
    for i in 0..l.len()
        invariant
            0 <= i <= l.len(),
            forall|i_: int| 0 <= i_ < i ==> l[i_] < t
    {
        if l[i] >= t {
            return false;
        }
    }
    return true;
}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_below_threshold() {
        assert!(below_threshold(vec![1, 2, 4, 10], 100));
        assert!(!below_threshold(vec![1, 20, 4, 10], 5));
        assert!(below_threshold(vec![1, 20, 4, 10], 21));
        assert!(below_threshold(vec![1, 20, 4, 10], 22));
        assert!(below_threshold(vec![1, 8, 4, 10], 11));
        assert!(!below_threshold(vec![1, 8, 4, 10], 10));
    }
}
