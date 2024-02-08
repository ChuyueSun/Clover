method append(a:array<int>, b:int) returns (c:array<int>)
  requires a.Length>0
  ensures  c.Length == a.Length +1
{
  c := new int[a.Length+1];
  var i:= 0;
  while (i < a.Length)
    invariant 0 <= i <= a.Length
    invariant forall ii::0<=ii<i ==> c[ii]==a[ii]
  {
    c[i] := a[i];
    i:=i+1;
  }
  c[a.Length]:=b;
}
