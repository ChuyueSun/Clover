method minArray(a: array<int>) returns (r:int)
  requires a.Length > 0
  ensures forall i :: 0 <= i < a.Length ==> r <= a[i]
{
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
