method minArray(a: array<int>) returns (r:int)

{
  assert a.Length>0;
  r:=1000;
  var i:=0;
  while i<a.Length
    invariant 0 <= i <= a.Length
    invariant forall x :: 0 <= x < i ==> r <= a[x]
  {
    if r>a[i]{
      r:=a[i];
    }
    i:=i+1;
  }
}
