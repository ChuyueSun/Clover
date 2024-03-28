method SumOfDivisors(n: nat) returns (sum: nat)
    requires n > 0
    ensures sum >= n
{
    sum := 0;
    for i := 1 to n + 1
        invariant 1 <= i <= n + 1
        invariant sum >= i - 1
    {
        if (n % i == 0) {
            sum := sum + i;
        }
    }
}