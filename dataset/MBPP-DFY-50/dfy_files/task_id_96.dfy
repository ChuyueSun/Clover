method NumberOfDivisors(n: int) returns (count: int)
    requires n > 0
    ensures count >= 1
    ensures count == | set i: int | 1 <= i <= n && n % i == 0 |
{
    count := 0;
    var i := 1;
    while i <= n
        invariant 1 <= i
        invariant count == | set k: int | 1 <= k < i && n % k == 0 |
    {
        if n % i == 0
        {
            count := count + 1;
        }
        i := i + 1;
    }
}