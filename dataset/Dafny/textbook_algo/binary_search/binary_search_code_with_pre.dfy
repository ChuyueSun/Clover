method BinarySearch(a: array<int>, key: int) returns (n: int)

{
  assert forall i,j :: 0<=i<j<a.Length ==> a[i]<=a[j];
  var lo, hi := 0, a.Length;
  while lo<hi
    invariant 0<= lo <= hi <= a.Length
    invariant forall i :: 0<=i<lo ==> a[i] < key
    invariant forall i :: hi<=i<a.Length ==> a[i] >= key
  {
    var mid := (lo + hi) / 2;
    if a[mid] < key {
      lo := mid + 1;
    } else {
      hi := mid;
    }
  }
  n:=lo;
}
