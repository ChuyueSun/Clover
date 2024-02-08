method Sum() returns (s:int, n:int)
  ensures s == n * (n + 1) / 2
  ensures n==33
{
  n, s := 0, 0;
  while n != 33
    invariant s == n * (n + 1) / 2
  {
    n := n + 1;
    s := s + n;
  }
}
