method LongestSublistLength(l: seq<seq<int>>) returns (longestLength: int)
    ensures longestLength >= 0
    ensures forall i :: 0 <= i < |l| ==> |l[i]| <= longestLength
    ensures exists i :: 0 <= i < |l| && |l[i]| == longestLength
{
    longestLength := 0;
    for i := 0 to |l|
        invariant 0 <= i <= |l|
        invariant longestLength >= 0
        invariant forall j :: 0 <= j < i ==> |l[j]| <= longestLength
        invariant exists j :: 0 <= j < i && |l[j]| == longestLength
    {
        if |l[i]| > longestLength {
            longestLength := |l[i]|;
        }
    }
}