method SelectionSort(a: array<int>)
{
  var n:= 0;
  while n != a.Length
 {
    var mindex, m := n, n+1;
    while m != a.Length
   {
      if a[m] < a[mindex] {
        mindex := m;
      }
      m := m+1;
    }
    a[n], a[mindex] := a[mindex], a[n];
    n := n+1;
  }

}
