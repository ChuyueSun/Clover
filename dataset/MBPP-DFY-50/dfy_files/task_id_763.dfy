method MinDifference(a: array<int>) returns (minDiff: int)
    requires a.Length >= 2
    ensures forall i, j :: 0 <= i < j < a.Length ==> minDiff <= a[j] - a[i]
    ensures exists i, j :: 0 <= i < j < a.Length && minDiff == a[j] - a[i]
{
    minDiff := a[1] - a[0];
    for i := 0 to a.Length - 2
        invariant 0 <= i < a.Length
        invariant forall k, l :: 0 <= k < l < i+1 ==> minDiff <= a[l] - a[k]
        invariant exists k, l :: 0 <= k < l < i+1 && minDiff == a[l] - a[k]
    {
        for j := i+1 to a.Length
            invariant i+1 <= j <= a.Length
            invariant forall k, l :: i+1 <= k < l < j ==> minDiff <= a[l] - a[k]
            invariant (forall k :: i+1 <= k < j ==> minDiff <= a[k] - a[i]) && (exists k :: i+1 <= k < j && minDiff == a[k] - a[i])
        {
            if a[j] - a[i] < minDiff
            {
                minDiff := a[j] - a[i];
            }
        }
    }
}