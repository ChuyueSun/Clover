method SumOfNonRepeatingElements(a: array<int>) returns (sum: int)
    requires a != null
    ensures sum == SumOfNonRepeatingElementsInSeq(a[..])
{
    var seen: set<int> := {};
    var duplicates: set<int> := {};
    sum := 0;

    for i := 0 to a.Length
        invariant 0 <= i <= a.Length
        invariant seen == (set k | 0 <= k < i :: a[k])
        invariant duplicates == (set k | 0 <= k < i && a[k] in seen :: a[k])
        invariant sum == SumOfNonRepeatingElementsInSeq(a[..i])
    {
        if a[i] in seen {
            duplicates := duplicates + {a[i]};
        } else {
            seen := seen + {a[i]};
        }
        if !(a[i] in duplicates) {
            sum := sum + a[i];
            SumOfNonRepeatingElementsInSeqLemma(a[..i], a[i]);
        }
    }
}

ghost function SumOfNonRepeatingElementsInSeq(s: seq<int>): int
{
    if |s| == 0 then 0
    else if Count(s, s[0]) > 1 then SumOfNonRepeatingElementsInSeq(s[1..])
    else s[0] + SumOfNonRepeatingElementsInSeq(s[1..])
}

ghost function Count(s: seq<int>, x: int): nat
{
    if |s| == 0 then 0
    else (if s[0] == x then 1 else 0) + Count(s[1..], x)
}

lemma SumOfNonRepeatingElementsInSeqLemma(s: seq<int>, x: int)
    requires Count(s, x) == 0
    ensures SumOfNonRepeatingElementsInSeq(s + [x]) == SumOfNonRepeatingElementsInSeq(s) + x
{
}