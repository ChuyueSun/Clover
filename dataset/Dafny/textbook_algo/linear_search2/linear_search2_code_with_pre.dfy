method LinearSearch(a: array<int>, e: int) returns (n:int)

{
  assert exists i::0<=i<a.Length && a[i]==e;

  n :=0;
  while n!=a.Length
    invariant 0<=n<=a.Length
    invariant forall i::0<=i<n ==> e!=a[i]
  {
    if e==a[n]{
      return;
    }
    n:=n+1;
  }
}
