method JoinIntegers(a: array<int>) returns (result: int)
    requires a != null
    requires a.Length > 0
    requires forall i :: 0 <= i < a.Length ==> 0 <= a[i] < 10
    ensures result == ArrayToInt(a, a.Length)
{
    result := 0;
    var multiplier := 1;
    for i := a.Length - 1; i >= 0; i := i - 1
        invariant 0 <= i < a.Length
        invariant result == ArrayToInt(a, a.Length) - ArrayToInt(a, i) * multiplier
        invariant multiplier == 10^(a.Length - i - 1)
    {
        result := result + a[i] * multiplier;
        multiplier := multiplier * 10;
    }
}

function ArrayToInt(a: array<int>, n: int): int
    requires a != null
    requires 0 <= n <= a.Length
    requires forall i :: 0 <= i < n ==> 0 <= a[i] < 10
    decreases n
{
    if n == 0 then 0
    else ArrayToInt(a, n - 1) * 10 + a[n - 1]
}