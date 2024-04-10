method rotate(a: array<int>, offset:int) returns (b: array<int> )
  requires 0<=offset<=a.Length
  ensures b.Length==a.Length
{
  b:= new int[a.Length];
  var i:=0;
  while i<a.Length
    invariant 0<=i<=a.Length
  {
    var dest:= (i-offset)%a.Length;
    b[dest]:=a[i];
    i:=i+1;
  }
}
