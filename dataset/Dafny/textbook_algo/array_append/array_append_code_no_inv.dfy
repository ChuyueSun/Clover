method append(a:array<int>, b:int) returns (c:array<int>)
{
  c := new int[a.Length+1];
  var i:= 0;
  while (i < a.Length)
  {
    c[i] := a[i];
    i:=i+1;
  }
  c[a.Length]:=b;

}
