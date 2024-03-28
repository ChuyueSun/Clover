method IsSumOfPowersOfTwo(n: int) returns (result: bool)
    requires n >= 0
    ensures result <==> (exists a: seq<int> :: (forall k :: 0 <= k < |a| ==> 2^a[k] > 0) && n == sum(a[k] | k in 0..|a|) )
{
    result := true;
    if n == 0 {
        result := false;
    } else {
        var i := 0;
        while (1 << i <= n)
            invariant n >= 0
            invariant i >= 0
        {
            if ((n & (1 << i)) != 0) {
                n := n - (1 << i);
            }
            i := i + 1;
        }
        result := n == 0;
    }
}