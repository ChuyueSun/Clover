method Sum() returns (s:int, n:int)

{
  n, s := 0, 0;
  while n != 33
    invariant s == n * n
  {
    n := n + 1;
    s := s + n * 2 - 1;
  }
}
