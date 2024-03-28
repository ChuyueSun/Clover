method ReverseString(s: string) returns (v: string)
    ensures |v| == |s|
    ensures forall i :: 0 <= i < |s| ==> v[i] == s[|s| - 1 - i]
    {
        v := "";
        for i := |s| - 1 downto 0
        invariant 0 <= i < |s|
        invariant |v| == |s| - 1 - i
        invariant forall k :: 0 <= k < |s| - 1 - i ==> v[k] == s[|s| - 1 - k]
        {
            v := v + s[i];
        }
    }

    method ReverseEachString(s: seq<string>) returns (v: seq<string>)
    ensures |v| == |s|
    ensures forall i :: 0 <= i < |s| ==> |v[i]| == |s[i]| && forall j :: 0 <= j < |s[i]| ==> v[i][j] == s[i][|s[i]| - 1 - j]
    {
        v := [];
        for i := 0 to |s|
        invariant 0 <= i <= |s|
        invariant |v| == i
        invariant forall k :: 0 <= k < i ==> |v[k]| == |s[k]| && forall j :: 0 <= j < |s[k]| ==> v[k][j] == s[k][|s[k]| - 1 - j]
        {
            var reversed := ReverseString(s[i]);
            v := v + [reversed];
        }
    }