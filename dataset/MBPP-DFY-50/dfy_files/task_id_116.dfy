method ConvertSeqToInt(s: seq<int>) returns (result: int)
        requires forall i :: 0 <= i < |s| ==> 0 <= s[i] <= 9
        ensures result >= 0
    {
        result := 0;
        for i := 0 to |s|
            invariant 0 <= i <= |s|
            invariant 0 <= result
            invariant result == SeqToInt(s[..i])
        {
            result := result * 10 + s[i];
        }
    }


    function SeqToInt(s: seq<int>) : int
        requires forall i :: 0 <= i < |s| ==> 0 <= s[i] <= 9
        ensures 0 <= result
    {
        if |s| == 0 then 0
        else SeqToInt(s[..|s|-1]) * 10 + s[|s|-1]
    }