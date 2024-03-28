predicate IsPrime(n: int)
    {
        if n < 2 then false
        else if n == 2 then true
        else if n % 2 == 0 then false
        else {
            var i := 3;
            while (i * i <= n)
                invariant 2 <= i
                invariant (forall k :: 2 <= k < i ==> n % k != 0)
            {
                if n % i == 0 then return false;
                i := i + 2;
            }
            true
        }
    }

    method CountPrimes(n: int) returns (count: int)
    requires n >= 0
    ensures count >= 0
    ensures count == | set i: int | 2 <= i < n && IsPrime(i)|
{
    count := 0;
    var i := 2;
    while i < n
        invariant 2 <= i <= n
        invariant count == | set k: int | 2 <= k < i && IsPrime(k)|
    {
        if IsPrime(i) {
            count := count + 1;
        }
        i := i + 1;
    }
}