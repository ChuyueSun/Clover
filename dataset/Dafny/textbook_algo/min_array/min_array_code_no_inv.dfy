method minArray(a: array<int>) returns (r:int)
{
  r:=a[0];
  var i:=1;
  while i<a.Length
 {
    if r>a[i]{
      r:=a[i];
    }
    i:=i+1;
  }
}
