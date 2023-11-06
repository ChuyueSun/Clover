method remove_front(a:array<int>) returns (c:array<int>)

{
  c := new int[a.Length-1];
  var i:= 1;
  while (i < a.Length)
 {
    c[i-1] := a[i];
    i:=i+1;
  }
}
