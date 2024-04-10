method append(a:array<int>, b:int) returns (c:array<int>)

{
  c := new int[a.Length+1];
  var i:= 0;
  while (i < a.Length)
    invariant 0 <= i <= a.Length
  {
    c[i] := 1;
    i:=i+1;
  }
  c[a.Length]:=b;
}
