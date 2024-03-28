method ReplaceSpacesWithPercent20(s: string) returns (v: string)
    ensures |v| >= |s|
    ensures forall i, j :: 0 <= i < |s| && 0 <= j < |v| && s[i] == ' ' ==> (v[j] == '%' && v[j+1] == '2' && v[j+2] == '0')
    ensures forall i, j :: 0 <= i < |s| && 0 <= j < |v| && s[i] != ' ' ==> (v[j] == s[i])
{
    var s' : string := [];
    for i := 0 to |s|
    invariant 0 <= i <= |s|
    invariant |s'| >= i
    invariant forall k, l :: 0 <= k < i && 0 <= l < |s'| && s[k] == ' ' ==> (s'[l] == '%' && s'[l+1] == '2' && s'[l+2] == '0')
    invariant forall k, l :: 0 <= k < i && 0 <= l < |s'| && s[k] != ' ' ==> (s'[l] == s[k])
    {
        if s[i] == ' '
        {
            s' := s' + ['%', '2', '0'];
        }
        else 
        {
            s' := s' + [s[i]];
        }
    }
    return s';
}