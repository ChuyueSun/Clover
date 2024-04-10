method rotate(a: array<int>, offset:int) returns (b: array<int> )
  requires 0<=offset
  ensures b.Length==a.Length
  ensures forall  i::0<=i<a.Length ==>  b[i]==a[(i+offset)%a.Length]
{
  b:= new int[a.Length];
  var i:=0;
  while i<a.Length
    invariant 0<=i<=a.Length
    invariant forall j ::0<=j<i ==> b[j]==a[(j+offset)%a.Length]
  {
    b[i]:=a[(i+offset)%a.Length];
    i:=i+1;
  }
}