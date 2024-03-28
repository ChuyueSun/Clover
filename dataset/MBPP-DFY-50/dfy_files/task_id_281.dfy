method AreElementsUnique(a: array<int>) returns (unique: bool)
    requires a != null
    ensures unique <==> forall i, j :: 0 <= i < j < a.Length ==> a[i] != a[j]
    ensures !unique ==> exists i, j :: 0 <= i < j < a.Length && a[i] == a[j]
{
    unique := true;
    for i := 0 to a.Length
        invariant 0 <= i <= a.Length
        invariant unique <==> forall k, l :: 0 <= k < l < i ==> a[k] != a[l]
        invariant !unique ==> exists k, l :: 0 <= k < l < i && a[k] == a[l]
    {
        for j := i+1 to a.Length
            invariant i < j <= a.Length
            invariant unique <==> forall k :: 0 <= k < j ==> a[i] != a[k]
            invariant !unique ==> exists k :: i < k < j && a[i] == a[k]
        {
            if a[i] == a[j]
            {
                unique := false;
                break;
            }
        }
        if(!unique) { break; }
    }
}