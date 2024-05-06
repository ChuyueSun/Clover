use vstd::prelude::*;
fn main() {}

/*
Return maximum element in the list.
*/

verus! {
fn maximum(nmbs: Vec<i32>) -> (r: i32)
    requires
        nmbs.len() > 0,
    ensures
        forall|i: int| 0 <= i < nmbs.len() ==> nmbs[i] <= r,
        exists|i: int| 0 <= i < nmbs.len() && r == nmbs[i],
{
    let mut max = nmbs[0];
    for i in 1..nmbs.len()
        invariant
            forall|j: int| 0 <= j < i ==> nmbs[j] <= max,
            exists|j: int| 0 <= j < i && max == nmbs[j],
    {
        if nmbs[i] > max {
            max = nmbs[i];
        }
    }
    max
}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum() {
        assert!(maximum(vec![1, 2, 3]) == 3);
        assert!(maximum(vec![5, 3, -5, 2, -3, 3, 9, 0, 124, 1, -10]) == 124);
    }
}
