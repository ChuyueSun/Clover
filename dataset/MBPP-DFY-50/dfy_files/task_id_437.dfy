method RemoveOddChars(s: string) returns (v: string)
    ensures |v| == (|s| + 1) / 2
    ensures forall i :: 0 <= i < |v| ==> v[i] == s[2 * i]
{
    var s' : string := [];
    for i := 0 to |s| by 2
    invariant 0 <= i <= |s|
    invariant |s'| == i / 2
    invariant forall k :: 0 <= k < |s'| ==> s'[k] == s[2 * k]
    {
        s' := s' + [s[i]];
    }
    return s';
}