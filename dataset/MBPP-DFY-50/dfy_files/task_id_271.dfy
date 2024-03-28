method SumOfFifthPowerOfEvenNumbers(n: int) returns (sum: int)
    requires n >= 0
    ensures sum == (12 * n * n * n * n * n + 30 * n * n * n * n + 20 * n * n * n - n) / 5
{
    sum := 0;
    var i := 2;
    for k := 1 to n + 1
        invariant 1 <= k <= n + 1
        invariant i == 2 * k
        invariant sum == (12 * (k - 1) * (k - 1) * (k - 1) * (k - 1) * (k - 1) + 30 * (k - 1) * (k - 1) * (k - 1) * (k - 1) + 20 * (k - 1) * (k - 1) * (k - 1) - (k - 1)) / 5
    {
        sum := sum + i * i * i * i * i;
        i := i + 2;
    }
}