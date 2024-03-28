predicate IsEven(n: int)
{
    n % 2 == 0
}

method SumEvenNumbersAtEvenPositions(a: array<int>) returns (sum: int)
    ensures sum == (if a.Length == 0 then 0 else sum(0, a.Length - 1 where i: int :: IsEven(i) && IsEven(a[i]) => a[i]) else 0)
{
    sum := 0;
    for i := 0 to a.Length
        invariant 0 <= i <= a.Length
        invariant sum == (if i == 0 then 0 else sum(0, i - 1 where k: int :: IsEven(k) && IsEven(a[k]) => a[k]) else 0)
    {
        if IsEven(i) && IsEven(a[i])
        {
            sum := sum + a[i];
        }
    }
}