method BellNumber(n: nat) returns (bell: int)
    requires n >= 0
    ensures bell >= 0
{
    var bellNumbers := new int[n+1, n+1];
    bellNumbers[0, 0] := 1;
    for i := 1 to n+1
    {
        bellNumbers[i, 0] := bellNumbers[i-1, i-1];
        for j := 1 to i
        {
            bellNumbers[i, j] := bellNumbers[i-1, j-1] + bellNumbers[i, j-1];
        }
    }
    bell := bellNumbers[n, 0];
}