method minArray(a: array<int>) returns (r:int)
  //TOFILL
{
  r:=a[0];
  var i:=1;
  while i<a.Length
    invariant 0 <= i <= a.Length
    invariant forall x :: 0 <= x < i ==> r <= a[x]
    invariant exists x :: 0 <= x < i && r == a[x]
  {
    if r>a[i]{
      r:=a[i];
    }
    i:=i+1;
  }
}
