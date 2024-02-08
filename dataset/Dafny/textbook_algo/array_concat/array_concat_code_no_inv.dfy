method concat(a:array<int>, b:array<int>) returns (c:array<int>)
{
  c := new int[a.Length+b.Length];
  var i:= 0;
  while (i < c.Length)
  {
    c[i] := if i<a.Length then a[i] else b[i-a.Length];
    i:=i+1;
  }

}
