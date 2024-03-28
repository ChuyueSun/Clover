predicate IsDigit(c: char)
{
    48 <= c as int <= 57
}

method MoveNumbersToEnd(s: string) returns (v: string)
    ensures |v| == |s|
    ensures forall i, j :: 0 <= i < j < |v| && IsDigit(v[i]) ==> IsDigit(v[j])
    ensures multiset(v[..]) == multiset(s[..])
    ensures forall n, m :: 0 <= n < m < |s| && !IsDigit(s[n]) && !IsDigit(s[m]) ==> 
            exists k, l :: 0 <= k < l < |v| && v[k] == s[n] && v[l] == s[m]
{
    var nonNumbers: seq<char> := [];
    var numbers: seq<char> := [];
    for i := 0 to |s|
        invariant 0 <= i <= |s|
        invariant multiset(nonNumbers + numbers) == multiset(s[..i])
        invariant forall n, m :: 0 <= n < m < |nonNumbers| ==> 
                exists k, l :: 0 <= k < l < i && !IsDigit(s[k]) && !IsDigit(s[l]) && nonNumbers[n] == s[k] && nonNumbers[m] == s[l]
    {
        if IsDigit(s[i]) {
            numbers := numbers + [s[i]];
        } else {
            nonNumbers := nonNumbers + [s[i]];
        }
    }
    v := nonNumbers + numbers;
}