use vstd::prelude::*;
fn main() {}

/*
Return 2^n modulo p (be aware of numerics).
*/

verus! {
spec fn Modp(n: nat, p: nat) -> nat
    decreases n
{
    if n == 0 {
        1
    } else {
        (Modp((n - 1) as nat, p) * 2) % p
    }
}

fn modp(n: u32, p: u32) -> (result: u32)
    requires
        p > 0,
        p <= 0x1000_0000,
    ensures
        result == Modp(n as nat, p as nat),
{
    let mut res = 1;
    let mut i = 0;
    while i < n
        invariant
            i <= n, 0 < p <= 0x1000_0000, res <= 0x1000_0000,
            res == Modp(i as nat, p as nat),
    {
        res = (res * 2) % p;
        i = i + 1;
    }
    res
}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modp() {
        assert!(modp(3, 5) == 3);
        assert!(modp(1101, 101) == 2);
        assert!(modp(0, 101) == 1);
        assert!(modp(3, 11) == 8);
        assert!(modp(100, 101) == 1);
        assert!(modp(30, 5) == 4);
        assert!(modp(31, 5) == 3);
    }
}
