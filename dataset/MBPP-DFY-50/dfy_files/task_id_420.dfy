method CubeSumOfEvenNumbers(n: int) returns (sum: int)
    requires n >= 0
    ensures sum == n * n * (n + 1) * (n + 1)
{
    sum := 0;
    var i := 0;
    for k := 0 to n
        invariant 0 <= k <= n
        invariant i == 2 * k
        invariant sum == k * k * (k + 1) * (k + 1)
    {
        sum := sum + i * i * i;
        i := i + 2;
    }
}