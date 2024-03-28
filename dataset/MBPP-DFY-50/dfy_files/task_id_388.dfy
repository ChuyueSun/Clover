method HighestPowerOf2(n: int) returns (result: int)
    requires n > 0
    ensures 1 <= result <= n
    ensures exists k :: 0 <= k && result == 2^k
    ensures forall k :: 0 <= k && 2^k > n ==> result < 2^k
{
    result := 1;
    while (result * 2 <= n)
        invariant 1 <= result <= n
        invariant exists k :: 0 <= k && result == 2^k
        invariant forall k :: 0 <= k && 2^k > n ==> result < 2^k
    {
        result := result * 2;
    }
}