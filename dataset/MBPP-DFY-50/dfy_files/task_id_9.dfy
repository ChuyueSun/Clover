method MinRotations(s: string) returns (rotations: int)
    requires |s| > 0
    ensures 0 <= rotations <= |s|
    ensures (rotations == 0) <==> (forall i :: 0 < i < |s| ==> s != RotateRight(s, i))
    ensures rotations > 0 ==> s == RotateRight(s, rotations)
{
    rotations := 0;
    var temp: string;

    for i := 1 to |s|
        invariant 0 <= i <= |s|
        invariant 0 <= rotations <= i
        invariant (rotations == 0) <==> (forall k :: 0 < k < i ==> s != RotateRight(s, k))
        invariant rotations > 0 ==> s == RotateRight(s, rotations)
    {
        temp := RotateRight(s, i);
        if temp == s {
            rotations := i;
            break;
        }
    }
}

method RotateRight(s: string, n: int) returns (r: string)
    requires n >= 0
    ensures |r| == |s|
    ensures forall i :: 0 <= i < |s| ==> r[i] == s[(i - n + |s|) % |s|]
{
    var rotated: string := "";
    for i := 0 to |s|
        invariant 0 <= i <= |s|
        invariant |rotated| == i
        invariant forall k :: 0 <= k < i ==> rotated[k] == s[(k - n + |s|) % |s|]
    {
        rotated := rotated + s[(i - n + |s|) % |s|];
    }
    return rotated;
}