predicate IsWhitespace(c: char)
{
    c == ' '
}

method RemoveWhitespaces(s: string) returns (v: string)
    ensures forall i :: 0 <= i < |v| ==> !IsWhitespace(v[i])
    ensures forall i, j :: 0 <= i < |s| && !IsWhitespace(s[i]) && 0 <= j < i && !IsWhitespace(s[j]) ==> v[|set k : int | 0 <= k < j && !IsWhitespace(s[k])|] == s[i]
{
    var s' : string := [];
    for i := 0 to |s|
    invariant 0 <= i <= |s|
    invariant forall k :: 0 <= k < |s'| ==> !IsWhitespace(s'[k])
    invariant forall k, j :: 0 <= k < i && !IsWhitespace(s[k]) && 0 <= j < k && !IsWhitespace(s[j]) ==> s'[|set l : int | 0 <= l < j && !IsWhitespace(s[l])|] == s[k]
    {
        if !IsWhitespace(s[i])
        {
            s' := s' + [s[i]];
        }
    }
    return s';
}