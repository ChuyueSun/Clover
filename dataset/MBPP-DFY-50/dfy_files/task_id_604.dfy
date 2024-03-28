function ReverseString(s: string): string
    ensures |s| == |ReverseString(s)|
    ensures forall i :: 0 <= i < |s| ==> s[i] == ReverseString(s)[|s| - 1 - i]
{
    var r := "";
    for i := |s| - 1 downto 0
    invariant 0 <= i < |s|
    invariant |r| == |s| - i - 1
    invariant forall j :: 0 <= j < |r| ==> r[j] == s[|s| - j - 1]
    {
        r := r + s[i];
    }
    return r;
}

method ReverseWords(s: string) returns (v: string)
    ensures v == ReverseString(s)
{
    var words := s.Split(' ');
    var reversedWords := "";
    for i := |words| - 1 downto 0
    invariant 0 <= i < |words|
    invariant forall j :: 0 <= j < |reversedWords| ==> reversedWords[j] == words[|words| - j - 1]
    {
        reversedWords := reversedWords + " " + words[i];
    }
    return reversedWords;
}