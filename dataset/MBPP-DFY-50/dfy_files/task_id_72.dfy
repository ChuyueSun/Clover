method IsDifferenceOfSquares(n: int) returns (result: bool)
    requires n >= 0
    ensures result == true ==> (exists i: int, j: int :: 0 <= j <= i && n == i*i - j*j)
    ensures result == false ==> (forall i: int, j: int :: 0 <= j <= i ==> n != i*i - j*j)
{
    result := false;
    var i := 0;
    while i * i <= n
        invariant 0 <= i
        invariant result == (exists j: int :: 0 <= j < i && n == i*i - j*j)
    {
        var j := 0;
        while j <= i
            invariant 0 <= j <= i
            invariant result == (exists k: int :: 0 <= k < j && n == i*i - k*k)
        {
            if n == i*i - j*j
            {
                result := true;
                break;
            }
            j := j + 1;
        }
        if result
        {
            break;
        }
        i := i + 1;
    }
}