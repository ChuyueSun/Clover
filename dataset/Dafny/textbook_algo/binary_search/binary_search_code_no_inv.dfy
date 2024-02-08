method BinarySearch(a: array<int>, key: int) returns (n: int)

{
  var lo, hi := 0, a.Length;
  while lo<hi
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
