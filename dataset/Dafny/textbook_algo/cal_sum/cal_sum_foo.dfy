method foo() returns (s:int, n:int)
  //TOFILL
{

  n, s := 0, 0;
  while n != 33
    invariant s == n * (n - 1) / 2
  {
    s := s + n;
    n := n + 1;
  }
}
