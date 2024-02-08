method LinearSearch(a: array<int>, e: int) returns (n:int)

{
  n :=0;
  while n!=a.Length
    invariant 0<=n<=a.Length
    invariant forall i::0<=i<n ==> a[i] < e
  {
    if a[n] >= e {
      return;
    }
    n:=n+1;
  }
}
