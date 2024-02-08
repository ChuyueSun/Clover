method foo(a: array<int>, b: array<int>) returns (c: array<int> )
  requires a.Length==b.Length
  ensures c.Length==a.Length
{
  c:= new int[a.Length];
  var i:=0;
  while i<a.Length
    invariant 0<=i<=a.Length
    invariant forall j:: 0 <= j< i==> a[j] + b[j]==c[j]
  {
    c[i]:=a[i]+b[i];
    i:=i+1;
  }
}
