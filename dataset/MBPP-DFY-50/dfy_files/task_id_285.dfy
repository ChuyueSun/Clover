method ContainsABBB(s: string) returns (result: bool)
    ensures result <==> exists i :: 0 <= i < |s| - 2 && s[i] == 'a' && (s[i + 1] == 'b' && s[i + 2] == 'b') && (i + 3 >= |s| || s[i + 3] != 'b')
{
    result := false;
    for i := 0 to |s| - 2
        invariant 0 <= i <= |s| - 2
        invariant result <==> exists k :: 0 <= k < i && s[k] == 'a' && (s[k + 1] == 'b' && s[k + 2] == 'b') && (k + 3 >= |s| || s[k + 3] != 'b')
    {
        if s[i] == 'a' && s[i + 1] == 'b' && s[i + 2] == 'b' && (i + 3 >= |s| || s[i + 3] != 'b') {
            result := true;
            break;
        }
    }
}