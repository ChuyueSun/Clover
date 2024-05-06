use vstd::prelude::*;
fn main() {}

/*
    You're given a list of deposit and withdrawal operations on a bank account that starts with
    zero balance. Your task is to detect if at any point the balance of account fallls below zero, and
    at that point function should return True. Otherwise it should return False.
*/


verus! {
// spec fn sum(v: Vec<i32>, start: int, end: int) -> (r: int)
//     recommends 0 <= start && start <= end && end <= v.len(),
//     decreases end - start
// {
//     if start == end {
//         0
//     } else {
//         v[start] + sum(v, start + 1, end)
//     }
// }
spec fn sum(v: Vec<i32>, start: int, len: nat) -> (r: int)
    recommends 0 <= start && start + len <= v.len(),
    decreases len
{
    if len == 0 {
        0
    } else {
        sum(v, start, (len - 1) as nat) + v[start + len as int - 1]
    }
}

fn below_zero(operations: Vec<i32>) -> (r: bool)
    requires
        forall|i: int| 0 <= i < operations.len() ==> -0x1fff_ffff <= #[trigger] operations[i] <= 0x1fff_ffff,
        forall|i: int| 0 <= i <= operations.len() ==> -0x1fff_ffff <= #[trigger] sum(operations, 0, i as nat) <= 0x1fff_ffff,
    ensures
        r == (exists |i: int| 0 <= i <= operations.len() && #[trigger] sum(operations, 0, i as nat) < 0),
{
    let mut balance: i32 = 0;
    for n in 0..operations.len()
        invariant
            0 <= n <= operations.len(),
            balance == sum(operations, 0, n as nat),
            forall|i: int| 0 <= i <= n ==> #[trigger] sum(operations, 0, i as nat) >= 0,
            // overflow checks
            -0x1fff_ffff <= balance <= 0x1fff_ffff,
            forall|i: int| 0 <= i < operations.len() ==> -0x1fff_ffff <= #[trigger] operations[i] <= 0x1fff_ffff,
            forall|i: int| 0 <= i <= operations.len() ==> -0x1fff_ffff <= #[trigger] sum(operations, 0, i as nat) <= 0x1fff_ffff,
    {
        let op = operations[n];
        assert(-0x1fff_ffff <= operations[n as int] <= 0x1fff_ffff);
        balance = balance + op;
        assert(balance == sum(operations, 0, (n + 1) as nat));
        assert(-0x1fff_ffff <= balance <= 0x1fff_ffff);
        if balance < 0 {
            return true;
        }
    }
    return false;
}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_below_zero() {
        assert_eq!(below_zero(vec![]), false);
        assert_eq!(below_zero(vec![1, 2, -3, 1, 2, -3]), false);
        assert_eq!(below_zero(vec![1, 2, -4, 5, 6]), true);
        assert_eq!(below_zero(vec![1, -1, 2, -2, 5, -5, 4, -4]), false);
        assert_eq!(below_zero(vec![1, -1, 2, -2, 5, -5, 4, -5]), true);
        assert_eq!(below_zero(vec![1, -2, 2, -2, 5, -5, 4, -4]), true);
    }
}
