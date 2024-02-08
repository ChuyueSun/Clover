method concat(a:array<int>, b:array<int>) returns (c:array<int>)
  //TOFILL
{
  c := new int[a.Length+b.Length];
  var i:= 0;
  while (i < c.Length)
    invariant 0 <= i <= c.Length
    invariant if i<a.Length then c[..i]==a[..i] else c[..i]==a[..]+b[..(i-a.Length)]
  {
    c[i] := if i<a.Length then a[i] else b[i-a.Length];
    i:=i+1;
  }

}
