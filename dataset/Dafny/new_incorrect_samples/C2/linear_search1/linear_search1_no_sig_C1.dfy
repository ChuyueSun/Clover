method foo(a: array<int>, e: int) returns (n:int)
  requires a.Length>0 && e>0
  ensures 0<=n<=a.Length
  ensures n==a.Length || a[n]==e
  ensures forall i::0<=i < n ==> e!=a[i]
{
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
