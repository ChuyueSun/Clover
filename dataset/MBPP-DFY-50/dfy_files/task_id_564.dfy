method CountDistinctPairs(a: array<int>) returns (count: int)
    requires a != null
    ensures count == (exists i, j :: 0 <= i < j < a.Length && a[i] != a[j])
{
    count := 0;
    for i := 0 to a.Length
        invariant 0 <= i <= a.Length
        invariant count == (exists k, l :: 0 <= k < l < i && a[k] != a[l])
    {
        for j := i+1 to a.Length
            invariant i+1 <= j <= a.Length
            invariant count == (exists k, l :: 0 <= k < l < j && a[k] != a[l])
        {
            if a[i] != a[j] {
                count := count + 1;
            }
        }
    }
}