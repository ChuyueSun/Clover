method LucasNumber(n: int) returns (ln: int)
    requires n >= 0
    ensures ln == if n == 0 then 2 else if n == 1 then 1 else LucasNumber(n - 1) + LucasNumber(n - 2)
{
    if n == 0 {
        ln := 2;
    } else if n == 1 {
        ln := 1;
    } else {
        var a := 2;
        var b := 1;
        for k := 2 to n
            invariant 2 <= k <= n + 1
            invariant a == LucasNumber(k - 2)
            invariant b == LucasNumber(k - 1)
        {
            var tmp := a;
            a := b;
            b := tmp + b;
        }
        ln := b;
    }
}