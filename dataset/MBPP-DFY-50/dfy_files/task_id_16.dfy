predicate IsLowerCase(c : char)
{
    97 <= c as int <= 122
}

method IsValidFormat(s: string) returns (result: bool)
    ensures result <==> (forall i :: 0 <= i < |s| ==> (IsLowerCase(s[i]) || s[i] == '_') && (i == 0 || i == |s| - 1 || s[i] != '_' || (s[i] == '_' && s[i-1] != '_' && s[i+1] != '_')))
{
    result := true;

    if |s| == 0 || s[0] == '_' || s[|s| - 1] == '_' {
        result := false;
        return;
    }

    for i := 0 to |s|
        invariant 0 <= i <= |s|
        invariant result <==> (forall k :: 0 <= k < i ==> (IsLowerCase(s[k]) || s[k] == '_') && (k == 0 || k == |s| - 1 || s[k] != '_' || (s[k] == '_' && s[k-1] != '_' && s[k+1] != '_')))
    {
        if !IsLowerCase(s[i]) && s[i] != '_' {
            result := false;
            break;
        }
        if i > 0 && i < |s| - 1 && s[i] == '_' && (s[i-1] == '_' || s[i+1] == '_') {
            result := false;
            break;
        }
    }
}