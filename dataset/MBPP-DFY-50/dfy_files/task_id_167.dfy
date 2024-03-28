method SmallestPowerOfTwo(n: int) returns (power: int)
    requires n > 0
    ensures power >= n && exists k :: power == 2^k
    ensures forall k :: (2^k < n) ==> power != 2^k
{
    power := 1;
    while power < n
        invariant power > 0
        invariant exists k :: power == 2^k
        invariant forall k :: (2^k < n) ==> power != 2^k
    {
        power := power * 2;
    }
}