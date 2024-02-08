method LinearSearch(a: array<int>, e: int) returns (n:int)
  requires exists i:: 0 <= i < a.Length && a[i] == e
  ensures 0 <= n < a.Length && a[n] == e
{
  var i := 0;
  while i < a.Length
    invariant 0 <= i <= a.Length
    invariant forall j :: 0 <= j < i ==> a[j] != e
  {
    if a[i] == e {
      return i;
    }
    i := i + 1;
  }
  return -1;
}
