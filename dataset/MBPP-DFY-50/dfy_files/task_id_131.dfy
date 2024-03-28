predicate IsVowel(c: char)
{
    c in {'a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'}
}

method ReverseVowels(s: string) returns (v: string)
    ensures |v| == |s|
    ensures forall i :: 0 <= i < |s| ==> (IsVowel(s[i]) ==> s[i] in v && v[i] in s && v[i] != s[i]) && (!IsVowel(s[i]) ==> v[i] == s[i])
{
    var vowels : seq<char> := [];
    for i := 0 to |s|
    invariant 0 <= i <= |s|
    invariant forall k :: 0 <= k < i && IsVowel(s[k]) ==> s[k] in vowels
    {
        if IsVowel(s[i])
        {
            vowels := [s[i]] + vowels;
        }
    }

    var j := 0;
    var s' : string := [];
    for i := 0 to |s|
    invariant 0 <= i <= |s|
    invariant |s'| == i
    invariant forall k :: 0 <= k < i && IsVowel(s[k]) ==> s'[k] in vowels
    invariant forall k :: 0 <= k < i && !IsVowel(s[k]) ==> s[k] == s'[k]
    {
        if IsVowel(s[i])
        {
            s' := s' + [vowels[j]];
            j := j + 1;
        }
        else 
        {
            s' := s' + [s[i]];
        }
    }
    return s';
}