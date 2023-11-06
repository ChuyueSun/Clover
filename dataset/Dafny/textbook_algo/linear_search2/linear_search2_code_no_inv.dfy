method LinearSearch(a: array<int>, e: int) returns (n:int)

{

  n :=0;
  while n!=a.Length
  {
    if e==a[n]{
      return;
    }
    n:=n+1;
  }
}
