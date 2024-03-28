method ExtractStrings(s: seq<string>, size: int) returns (v: seq<string>)
    requires size >= 0
    ensures forall str :: str in v ==> |str| == size
    ensures forall i, j :: 0 <= i < j < |v| ==> (exists k, l :: 0 <= k < l < |s| && |s[k]| == size && |s[l]| == size && v[i] == s[k] && v[j] == s[l])
{
    v := [];
    for i := 0 to |s|
    invariant 0 <= i <= |s|
    invariant forall str :: str in v ==> |str| == size
    invariant forall i, j :: 0 <= i < j < |v| ==> (exists k, l :: 0 <= k < l < i && |s[k]| == size && |s[l]| == size && v[i] == s[k] && v[j] == s[l])
    {
        if |s[i]| == size
        {
            v := v + [s[i]];
        }
    }
}