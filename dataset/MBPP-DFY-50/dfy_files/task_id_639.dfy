predicate IsUpperCase(s: string)
    requires |s| > 0
    {
        65 <= s[0] as int <= 90
    }

    method SumLengthOfNames(names: seq<string>) returns (sum: int)
    requires names != null
    ensures sum >= 0
    ensures sum == | set n: string | n in names && IsUpperCase(n) | 
    {
        var filteredNames := set n: string | n in names && IsUpperCase(n);
        sum := 0;
        for n in filteredNames
            invariant sum >= 0
            invariant sum <= |filteredNames|
        {
            sum := sum + |n|;
        }
    }