method rotate(a: array<int>, offset:int) returns (b: array<int> )
  //TOFILL
{
  b:= new int[a.Length];
  var i:=0;
  while i<a.Length
    invariant 0<=i<=a.Length
    invariant forall j::0<=j<i ==> if j+offset<a.Length then b[j+offset]==a[j] else b[j+offset-a.Length]==a[j]
  {
    var dest:= if offset+i<a.Length then offset+i else offset+i-a.Length;
    b[dest]:=a[i];
    i:=i+1;
  }
}
