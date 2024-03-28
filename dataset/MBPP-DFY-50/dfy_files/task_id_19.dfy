method ContainsDuplicate(a: array<int>) returns (result: bool)
    requires a != null
    ensures result ==> exists i, j :: 0 <= i < a.Length && 0 <= j < a.Length && i != j && a[i] == a[j]
    ensures !result ==> forall i, j :: 0 <= i < a.Length && 0 <= j < a.Length && i != j ==> a[i] != a[j]
{
    result := false;
    for i := 0 to a.Length
        invariant 0 <= i <= a.Length
        invariant !result ==> forall k, l :: 0 <= k < i && 0 <= l < i && k != l ==> a[k] != a[l]
    {
        for j := i + 1 to a.Length
            invariant i < j <= a.Length
            invariant !result ==> forall k :: i < k < j ==> a[i] != a[k]
        {
            if a[i] == a[j] {
                result := true;
                break;
            }
        }
        if result {
            break;
        }
    }
}