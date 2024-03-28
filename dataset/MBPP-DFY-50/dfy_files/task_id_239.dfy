method NumberOfSequences(m: int, n: int) returns (count: int)
    requires m > 0 && n > 0
    ensures count >= 0
{
    var dp := new int[m+1, n+1];
    for i := 0 to m+1
    {
        dp[i, 0] := 1;
    }
    for i := 1 to m+1
    {
        for j := 1 to n+1
        {
            var x := 0;
            for k := 0 to i/2
            {
                x := x + dp[k, j-1];
            }
            dp[i, j] := x;
        }
    }
    count := dp[m, n];
}