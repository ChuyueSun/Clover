method FindMaxProductPair(a: array<int>) returns (index1: int, index2: int)
    requires a != null
    requires a.Length >= 2
    ensures 0 <= index1 < index2 < a.Length
    ensures forall i, j :: 0 <= i < j < a.Length ==> a[index1] * a[index2] >= a[i] * a[j]
{
    index1 := 0;
    index2 := 1;
    for i := 0 to a.Length
        invariant 0 <= i <= a.Length
        invariant 0 <= index1 < index2 < a.Length
        invariant forall k, l :: 0 <= k < l < i ==> a[index1] * a[index2] >= a[k] * a[l]
    {
        for j := i + 1 to a.Length
            invariant i < j <= a.Length
            invariant 0 <= index1 < index2 < a.Length
            invariant forall k, l :: 0 <= k < l < j ==> a[index1] * a[index2] >= a[k] * a[l]
        {
            if a[i] * a[j] > a[index1] * a[index2]
            {
                index1 := i;
                index2 := j;
            }
        }
    }
}