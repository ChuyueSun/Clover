method rotate(a: array<int>, offset:int) returns (b: array<int>)
  requires a.Length!=0
  requires 0<=offset<a.Length
  ensures b.Length==a.Length
  ensures forall i::0<=i<a.Length ==> if i+offset<a.Length then b[i+offset]==a[i] else b[i+offset-a.Length]==a[i]
{
  var b := new int[a.Length];
  if offset == 0
  {
    b := a;
  }
  else
  {
    var i := 0;
    while i < a.Length
      invariant 0 <= i<=a.Length
      invariant forall j::0<=j<i ==> if j+offset<a.Length then b[j+offset]==a[j] else b[j+offset-a.Length]==a[j]
    {
      if i + offset < a.Length
      {
        b[i + offset] := a[i];
      }
      else
      {
        b[i + offset - a.Length] := a[i];
      }
      i := i + 1;
    }
  }
  return b;
}
