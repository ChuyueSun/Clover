method SelectionSort(a: array<int>)
  modifies a
  ensures forall i,j :: 0 <= i < j < a.Length ==> a[i] <= a[j]
  ensures multiset(a[..]) == old(multiset(a[..]))
{
  var n := a.Length;
  var i := 0;
  while(i < n-1)
    invariant 0 <= i <= n-1
    invariant forall k :: 0 <= k < i ==> a[k] <= a[i]
    invariant forall k :: i < k < n ==> a[i] <= a[k]
  {
    var minIndex := i;
    var j := i + 1;
    while(j < n)
      invariant i < j <= n
      invariant 0 <= minIndex <= i < n
      invariant forall k :: 0 <= k < i ==> a[k] <= a[minIndex]
      invariant forall k :: i < k < j ==> a[minIndex] <= a[k]
      invariant forall k :: j <= k < n ==> a[minIndex] <= a[k]
    {
      if(a[j] < a[minIndex])
      {
        minIndex := j;
      }
      j := j + 1;
    }
    var temp := a[i];
    a[i] := a[minIndex];
    a[minIndex] := temp;
    i := i + 1;
  }
}
