method Reverse(n: int) returns (result: int)
    requires n >= 0
    ensures result >= 0
    {
        result := 0;
        while n > 0
            invariant n >= 0
            invariant result >= 0
        {
            result := result * 10 + n % 10;
            n := n / 10;
        }
    }

    method IsOneLessThanTwiceReverse(n: int) returns (result: bool)
    requires n >= 0
    ensures result <==> n == 2 * Reverse(n) - 1
    {
        result := n == 2 * Reverse(n) - 1;
    }