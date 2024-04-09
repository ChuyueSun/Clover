method Sum(N:int) returns (s:int)
  requires N >= 0
  ensures s == N * (N + 1) / 2
{
  var n := 0;
  s := 0;
  while n != N
    invariant 0 <= n <= N
    invariant s == n * (n + 1) / 2
  {
    n := n + 1;
    s := s + n;
  }
}

method TestSum(){
  var s := Sum(33);
  print(s);
  s := Sum(10);
  print(s);
  s := Sum(0);
  print(s);
  s := Sum(4);
  print(s);
  s := Sum(7);
  print(s);
}

method Main(){
  TestSum();
}
