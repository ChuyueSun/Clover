method LinearSearch3<T>(a: array<T>, P: T -> bool) returns (n: int)

{
  n := 0;
  while true
  {
    if P(a[n]) {
      return;
    }
    n := n + 1;
  }
}
