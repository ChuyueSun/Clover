method Sum() returns (s:int, n:int)
  ensures s == n * n
  ensures n==33
{
  n, s := 0, 0;
  while n != 33
    invariant s == n * n
  {
    n := n + 1;
    s := s + n * 2 - 1;
  }
}
