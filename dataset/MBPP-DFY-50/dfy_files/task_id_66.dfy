predicate IsPositive(x: int)
{
    x > 0
}

method CountPositive(a: array<int>) returns (count: int)
    requires a != null
    ensures count >= 0
    ensures count == | set i: int | 0 <= i < a.Length && IsPositive(a[i])|
{
    count := 0;
    for i := 0 to a.Length
        invariant 0 <= i <= a.Length
        invariant count == | set j: int | 0 <= j < i && IsPositive(a[j])|
    {
        if IsPositive(a[i])
        {
            count := count + 1;
        }
    }
}