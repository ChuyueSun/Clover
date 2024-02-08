method Sum() returns (s:int, n:int)
  //TOFILL
{
  n, s := 0, 0;
  while n != 33
    invariant s == n * (n + 1) / 2
  {
    n := n + 1;
    s := s + n;
  }
}
