method arrayProduct(a: array<int>, b: array<int>) returns (c: array<int> )

{
  c:= new int[a.Length];
  var i:=0;
  while i<a.Length
  {
    c[i]:=a[i]*b[i];
    i:=i+1;
  }
}
