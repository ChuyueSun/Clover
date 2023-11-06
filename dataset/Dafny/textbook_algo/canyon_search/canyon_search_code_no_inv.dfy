method CanyonSearch(a: array<int>, b: array<int>) returns (d:nat)
{
  var m,n:=0,0;
  d:=if a[0] < b[0] then (b[0]-a[0]) else (a[0]-b[0]);
  while m<a.Length && n<b.Length
  {
    var t := if a[m] < b[n] then (b[n]-a[m]) else (a[m]-b[n]);
    d:=if t<d then t else d;
    if
    case a[m]<=b[n] =>
      m:=m+1;
    case b[n]<=a[m] =>
      n:=n+1;

  }
}

