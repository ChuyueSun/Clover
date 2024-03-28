method FindFirstNonRepeatedChar(s: string) returns (found: bool, c: char)
    ensures found ==> exists i :: 0 <= i < |s| && s[i] == c && (forall j :: 0 <= j < |s| && j != i ==> s[j] != c)
    ensures !found ==> forall i, j :: 0 <= i < j < |s| ==> s[i] == s[j]
{
    found := false;
    c := ' ';
    var i := 0;
    while i < |s| && !found
        invariant 0 <= i <= |s|
        invariant found ==> exists ii :: 0 <= ii < i && s[ii] == c && (forall jj :: 0 <= jj < i && jj != ii ==> s[jj] != c)
        invariant !found ==> forall ii, jj :: 0 <= ii < jj < i ==> s[ii] == s[jj]
    {
        var j := 0;
        var repeated := false;
        while j < |s| && !repeated
            invariant 0 <= j <= |s|
            invariant repeated ==> s[i] == s[j] && i != j
            invariant !repeated ==> forall jj :: 0 <= jj < j && jj != i ==> s[jj] != s[i]
        {
            if i != j && s[i] == s[j] {
                repeated := true;
            }
            j := j + 1;
        }
        if !repeated {
            found := true;
            c := s[i];
        }
        i := i + 1;
    }
}