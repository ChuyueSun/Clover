method remove_front(a:array<int>) returns (c:array<int>)
  requires a.Length>0
  ensures  a[0..a.Length - 1] == c[..]
{
  c := new int[a.Length-1];
  var i:= 0;
  while (i < a.Length - 1)
    invariant 0 <= i < a.Length
    invariant forall ii::0<=ii<i ==> c[ii]==a[ii]
  {
    c[i] := a[i];
    i:=i+1;
  }
}
