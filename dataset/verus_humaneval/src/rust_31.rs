use vstd::prelude::*;
fn main() {}

/*
Return true if a given number is IsPrime, and false otherwise.
*/

verus! {
// reference: https://github.com/verus-lang/verus/blob/55b24eea93ad3dc907a4219e8b58a2f5f3f43afb/source/rust_verify/example/rw2022_script.rs#L36C1-L69C2

spec fn divides(factor: nat, candidate: nat) -> bool {
    candidate % factor == 0
}

spec fn IsPrime(candidate: nat) -> bool {
    1 < candidate &&
    forall|factor: nat| 1 < factor < candidate ==> !divides(factor, candidate)
}

fn is_prime(n: u32) -> (result: bool)
    requires
        1 < n,
    ensures
        result == IsPrime(n as nat),
{
    if n < 2 {
        return false;
    }
    for k in 2..n
        invariant
            1 < k <= n,
            forall|smallerfactor: nat|
                1 < smallerfactor < k ==> !divides(smallerfactor, n as nat),
    {
        assert(1 < k < n);
        if n % k == 0 {
            assert(divides(k as nat, n as nat));
            assert(!IsPrime(n as nat));
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
    fn test_is_prime() {
        assert!(is_prime(6) == false);
        assert!(is_prime(101) == true);
        assert!(is_prime(11) == true);
        assert!(is_prime(13441) == true);
        assert!(is_prime(61) == true);
        assert!(is_prime(4) == false);
        assert!(is_prime(1) == false);
        assert!(is_prime(5) == true);
        assert!(is_prime(11) == true);
        assert!(is_prime(17) == true);
        assert!(is_prime(5 * 17) == false);
        assert!(is_prime(11 * 7) == false);
        assert!(is_prime(13441 * 19) == false);
    }
}
