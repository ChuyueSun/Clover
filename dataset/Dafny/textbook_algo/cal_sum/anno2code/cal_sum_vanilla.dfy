method Sum() returns (s:int, n:int)
  ensures s == n * (n - 1) / 2
  ensures n==33
{
  var i := 0;
  s := 0;
  while i < n
    invariant 0 <= i <= n
    invariant s == i * (i - 1) / 2
    decreases n - i
  {
    s := s + i;
    i := i + 1;
  }
}
