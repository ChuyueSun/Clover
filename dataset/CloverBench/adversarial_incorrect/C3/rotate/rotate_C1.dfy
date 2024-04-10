method rotate(a: array<int>, offset:int) returns (b: array<int> )
  requires 0<offset<a.Length
  ensures b.Length==a.Length
  ensures forall i ::0<=i<a.Length ==>  b[(i+offset)%a.Length]==a[i]
{
  b:= new int[a.Length];
  var i:=0;
  while i<a.Length
    invariant 0<=i<=a.Length
    invariant forall j::0<=j<i ==> b[(j+offset)%a.Length]==a[j]
  {
    assert (i+offset)%a.Length == (if i+offset>=a.Length then i+offset-a.Length else i+offset);
    var dest:= (i+offset)%a.Length;
    b[dest]:=a[i];
    i:=i+1;
  }
}
