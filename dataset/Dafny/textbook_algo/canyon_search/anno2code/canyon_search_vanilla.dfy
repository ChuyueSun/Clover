```Dafny
function Dist(x:int, y:int): nat{
    if x<y then y-x else x-y
}

method CanyonSearch(a: array<int>, b: array<int>) returns (d:nat)
requires a.Length !=0 && b.Length!=0
requires forall i,j :: 0<=i<j<a.Length ==> a[i]<=a[j]
requires forall i,j :: 0<=i<j<b.Length ==> b[i]<=b[j]
ensures exists i,j:: 0<=i<a.Length && 0<=j<b.Length && d==Dist(a[i],b[j])
ensures forall i,j:: 0<=i<a.Length && 0<=j<b.Length ==> d<=Dist(a[i],b[j])
{
    var minDist : nat := Dist(a[0], b[0]);
    var i : int := 0;
    var j : int := 0;

    while i < a.Length && j < b.Length
        invariant 0 <= i <= a.Length
        invariant 0 <= j <= b.Length
        invariant forall k1 : int, k2 : int :: 0 <= k1 < i && 0 <= k2 < j ==> minDist <= Dist(a[k1], b[k2])
        invariant forall k1 : int :: 0 <= k1 < i ==> exists k2 : int :: 0 <= k2 < j && minDist == Dist(a[k1], b[k2])
        invariant forall k2 : int :: 0 <= k2 < j ==> exists k1 : int :: 0 <= k1 < i && minDist == Dist(a[k1], b[k2])
    {
        var currDist := Dist(a[i], b[j]);
        if currDist < minDist {
            minDist := currDist;
        }
        if a[i] < b[j] {
            i := i + 1;
        } else if b[j] < a[i] {
            j := j + 1;
        } else {
            break;
        }
    }

    d := minDist;
}
```