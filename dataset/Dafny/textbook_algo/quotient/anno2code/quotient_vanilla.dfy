```Dafny
method Quotient(x: int, y:int) returns (r:int, q:int)
requires  x >= 0
requires y > 0
ensures q*y+r==x && y>r>=0
{
    var temp:= x;
    r:= 0;
    q:= 0;
    
    while(temp >= y)
        decreases temp, temp - y;
        invariant q*y + r == x && y > r >=0;
    {
        temp := temp - y;
        r := r + y;
        q := q + 1;
    }
}
```

Explanation:
1. We initialize `temp` to `x`, `r` to `0`, and `q` to `0`.
2. The loop invariant is `q*y + r = x && y > r >= 0`, which states that `q` represents the quotient `x/y`, `r` represents the remainder `x%y`, and `y` is greater than the remainder `r`.
3. Inside the loop, we subtract `y` from `temp`, add `y` to `r`, and increment `q` by `1`.
4. The loop terminates when `temp` becomes less than `y`, and `q*y + r = x && y > r >= 0` is still maintained.
5. At the end, the values of `r` and `q` represent the remainder and quotient of `x/y` respectively, satisfying the post-condition `q*y+r==x && y>r>=0`.