function perrin(n: int): int
    requires n >= 0
    decreases n
    {
        if n < 3 then n else perrin(n - 2) + perrin(n - 3)
    }

    method PerrinSum(n: int) returns (sum: int)
    requires n >= 0
    ensures sum == sumPerrin(n)
    {
        sum := 0;
        for i := 0 to n
            invariant 0 <= i <= n
            invariant sum == sumPerrin(i)
        {
            sum := sum + perrin(i);
        }
    }

    function sumPerrin(n: int): int
    requires n >= 0
    decreases n
    {
        if n == 0 then 0 else perrin(n) + sumPerrin(n - 1)
    }