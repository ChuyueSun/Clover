method rotate(a: array<int>, offset:int) returns (b: array<int> )

{
  b:= new int[a.Length];
  var i:=0;
  while i<a.Length
  {
    var dest:= if offset+i<a.Length then offset+i else offset+i-a.Length;
    b[dest]:=a[i];
    i:=i+1;
  }
}
