use vstd::prelude::*;
fn main() {}

/*
Return n-th Fibonacci number.
*/

verus! {
spec fn Fibo(n: nat) -> nat
    decreases n
{
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        Fibo((n - 1) as nat) + Fibo((n - 2) as nat)
    }
}


proof fn lemma_fibo_is_monotonic(i: nat, j: nat)
    requires
        i <= j,
    ensures
        Fibo(i) <= Fibo(j),
    decreases j - i
{
   if i < 2 && j < 2 {
   } else if i == j {
   } else if i == j - 1 {
       reveal_with_fuel(Fibo, 2);
       lemma_fibo_is_monotonic(i, (j - 1) as nat);
   } else {
       lemma_fibo_is_monotonic(i, (j - 1) as nat);
       lemma_fibo_is_monotonic(i, (j - 2) as nat);
   }
}

fn fib(n: i32) -> (result: i32)
    requires
        n >= 0,
        Fibo(n as nat) <= 0x1000_0000,
    ensures
        result == Fibo(n as nat) as i32,
{
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    assert(Fibo((n - 1) as nat) <= 0x1000_0000) by {
        lemma_fibo_is_monotonic((n - 1) as nat, n as nat);
    }
    assert(Fibo((n - 2) as nat) <= 0x1000_0000) by {
        lemma_fibo_is_monotonic((n - 2) as nat, (n - 1) as nat);
    }
    return fib(n - 1) + fib(n - 2);
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
