use vstd::prelude::*;
fn main() {}

/*
Return n-th Fibonacci number.
*/

verus! {
// reference: https://github.com/verus-lang/verus/blob/55b24eea93ad3dc907a4219e8b58a2f5f3f43afb/source/rust_verify/example/nevd_script.rs#L61C1-L115C2

spec fn fibo(n: nat) -> nat
    decreases n
{
    if n == 0 { 0 } else if n == 1 { 1 }
    else { fibo((n - 2) as nat) + fibo((n - 1) as nat) }
}

proof fn lemma_fibo_is_monotonic(i: nat, j: nat)
    requires i <= j,
    ensures fibo(i) <= fibo(j),
    decreases j - i
{
    if i < 2 && j < 2 {
    } else if i == j {
    } else if i == j - 1 {
        reveal_with_fuel(fibo, 2);
        lemma_fibo_is_monotonic(i, (j - 1) as nat);
    } else {
        lemma_fibo_is_monotonic(i, (j - 1) as nat);
        lemma_fibo_is_monotonic(i, (j - 2) as nat);
    }
}

spec fn fibo_fits_u64(n: nat) -> bool {
    fibo(n) <= 0xffff_ffff_ffff_ffff
}

exec fn fib(n: u64) -> (result: u64)
    requires fibo_fits_u64(n as nat),
    ensures result == fibo(n as nat),
{
    if n == 0 {
        return 0;
    }
    let mut prev: u64 = 0;
    let mut cur: u64 = 1;
    let mut i: u64 = 1;
    while i < n
        invariant
            0 < i <= n,
            fibo_fits_u64(n as nat),
            fibo_fits_u64(i as nat),
            cur == fibo(i as nat),
            prev == fibo((i - 1) as nat),
    {
        i = i + 1;
        proof { lemma_fibo_is_monotonic(i as nat, n as nat); }
        let new_cur = cur + prev;
        prev = cur;
        cur = new_cur;
    }
    cur
}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        assert!(fib(10) == 55);
        assert!(fib(1) == 1);
        assert!(fib(8) == 21);
        assert!(fib(11) == 89);
        assert!(fib(12) == 144);
    }
}
