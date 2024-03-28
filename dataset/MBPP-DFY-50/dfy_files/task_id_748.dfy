predicate IsCapitalLetter(c: char)
{
    65 <= c as int <= 90
}

method SpaceCapitalWords(s: string) returns (v: string)
    ensures |v| >= |s|
    ensures forall i :: 0 <= i < |s| && !IsCapitalLetter(s[i]) ==> v[i] == s[i]
    ensures forall i :: 1 < i < |v| && IsCapitalLetter(v[i]) && v[i-1] != ' ' ==> v[i-1] == ' ' && v[i] == s[i-1]
{
    var s' : string := [s[0]];
    for i := 1 to |s|
    invariant 1 <= i <= |s|
    invariant |s'| >= i
    invariant forall k :: 0 < k < i && !IsCapitalLetter(s[k]) ==> s'[k] == s[k]
    invariant forall k :: 1 < k < |s'| && IsCapitalLetter(s'[k]) && s'[k-1] != ' ' ==> s'[k-1] == ' ' && s'[k] == s[k-1]
    {
        if IsCapitalLetter(s[i])
        {
            s' := s' + [' '] + [s[i]];
        }
        else 
        {
            s' := s' + [s[i]];
        }
    }
    return s';
}