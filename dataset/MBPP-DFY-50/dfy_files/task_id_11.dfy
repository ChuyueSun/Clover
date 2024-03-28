method RemoveFirstAndLastChar(s: string, c: char) returns (v: string)
    requires exists i, j :: 0 <= i < j < |s| && s[i] == c && s[j] == c
    ensures |v| <= |s| - 2
    ensures (exists i, j :: 0 <= i < j < |s| && s[i] == c && s[j] == c && forall k :: 0 <= k < |v| ==> v[k] == if k < i then s[k] else if k < j - 1 then s[k + 1] else s[k + 2])
{
    var firstIndex := -1;
    var lastIndex := -1;
    for i := 0 to |s|
    invariant 0 <= i <= |s|
    {
        if s[i] == c {
            if firstIndex == -1 {
                firstIndex := i;
            }
            lastIndex := i;
        }
    }
    assert firstIndex != -1 && lastIndex != -1 && firstIndex < lastIndex;

    var s' : string := [];
    for i := 0 to |s|
    invariant 0 <= i <= |s|
    invariant |s'| == if i <= firstIndex then i else if i <= lastIndex then i - 1 else i - 2
    {
        if i != firstIndex && i != lastIndex {
            s' := s' + [s[i]];
        }
    }
    return s';
}