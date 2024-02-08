method BinarySearch(a: array<int>, key: int) returns (n: int)
  requires forall i,j :: 0<=i<j<a.Length ==> a[i]<=a[j]
  ensures 0<= n <=a.Length
{
  var i := 0;
  while i < a.Length
    invariant 0 <= i <= a.Length
    invariant forall ii :: 0 <= ii < i ==> a[ii] != key
  {
    if a[i] == key {
      n := i;
      break;
    }
    i := i + 1;
  }
  n := a.Length;
}
