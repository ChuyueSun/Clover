method onlineMax(a: array<int>, x: int) returns (ghost m:int, p:int)
  requires 0<=x<a.Length
  requires a.Length!=0
  ensures 0<=p<a.Length
  ensures forall i::0<=i<x==> a[i]<=m
  ensures forall i::x<=i<p ==> a[i]<=m
{
  var max := a[0];
  var pos := 0;

  if (x == 0) {
    m := max;
    p := pos;
  }
  else {
    var i := 1;
    while(i < x)
      invariant 1<=i<=x;
      invariant forall j::0<=j<i ==> a[j] <= max;
    {
      if (a[i] > max) {
        max := a[i];
        pos := i;
      }
      i := i + 1;
    }

    m := max;
    p := pos;

    var j := x;
    while(j < a.Length)
      invariant x<=j<=a.Length;
      invariant forall k::x<=k<j ==> a[k] <= max;
    {
      if (a[j] > max) {
        max := a[j];
        pos := j;
      }
      j := j + 1;
    }
  }
}
