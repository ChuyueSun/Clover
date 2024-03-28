method CountWays(n: int, k: int) returns (count: int)
    requires n >= 0
    requires k >= 0
    ensures count >= 0
{
    if n == 0 then
    {
        count := 0;
    }
    else if n == 1 then
    {
        count := k;
    }
    else
    {
        var same := k;
        var diff := k * (k - 1);
        for i := 3 to n
            invariant same >= 0
            invariant diff >= 0
        {
            var temp := same;
            same := diff;
            diff := (same + diff) * (k - 1);
        }
        count := same + diff;
    }
}