method SlopeSearch(a: array2<int>, key: int) returns (m:int, n:int)
  requires forall i,j,j'::0<=i<a.Length0 && 0<=j<j'<a.Length1 ==> a[i,j]<=a[i,j']
  requires forall i,i',j::0<=i<i'<a.Length0 && 0<=j<a.Length1 ==> a[i,j]<=a[i',j]
  requires exists i,j :: 0<=i<a.Length0 && 0<=j<a.Length1 && a[i,j]==key
  ensures 0<=m<a.Length0 && 0<=n<a.Length1
  ensures a[m,n]==key
{
  var i := 0;
  var j := 0;
  while (i < a.Length0)
    invariant 0 <= i <= a.Length0
    invariant 0 <= j <= a.Length1
    invariant forall k :: 0 <= k < i ==> a[k, j-1] < key
    decreases a.Length0 - i
  {
    while (j > 0 && a[i, j-1] > key)
      invariant 0 <= i <= a.Length0
      invariant 0 <= j <= a.Length1
      invariant forall k :: 0 <= k < i ==> a[k, j] < key
      decreases j
    {
      j := j - 1;
    }
    if (a[i, j] == key) {
      m, n := i, j;
      return;
    }
    i := i + 1;
  }
  return;
}
