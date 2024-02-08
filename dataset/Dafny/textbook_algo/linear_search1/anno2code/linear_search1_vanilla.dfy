method LinearSearch(a: array<int>, e: int) returns (n:int)
  ensures 0<=n<=a.Length
  ensures n==a.Length || a[n]==e
  ensures n==a.Length ==> forall i::0<=i<a.Length ==> e!=a[i]
{
  var i := 0;
  while i < a.Length
    invariant 0 <= i <= a.Length
    invariant forall j::0<=j<i ==> a[j] != e
  {
    if a[i] == e {
      return i;
    }
    i := i + 1;
  }
  return a.Length;
}
