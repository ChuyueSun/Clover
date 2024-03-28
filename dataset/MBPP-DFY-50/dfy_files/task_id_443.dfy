method FindLargestNegative(a: array<int>) returns (found: bool, largestNeg: int)
    requires a != null
    ensures !found ==> forall i :: 0 <= i < a.Length ==> a[i] >= 0
    ensures found ==> largestNeg < 0 && forall i :: 0 <= i < a.Length ==> a[i] >= 0 || a[i] <= largestNeg
{
    found := false;
    for i := 0 to a.Length
        invariant 0 <= i <= a.Length
        invariant !found ==> forall k :: 0 <= k < i ==> a[k] >= 0
        invariant found ==> largestNeg < 0 && forall k :: 0 <= k < i ==> a[k] >= 0 || a[k] <= largestNeg
    {
        if a[i] < 0 && (!found || a[i] > largestNeg)
        {
            found := true;
            largestNeg := a[i];
        }
    }
}