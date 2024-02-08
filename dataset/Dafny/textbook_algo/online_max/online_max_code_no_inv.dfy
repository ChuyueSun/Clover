method onlineMax(a: array<int>, x: int) returns (ghost m:int, p:int)

{
  p:= 0;
  var best := a[0];
  var i:=1;
  while i<x
  {
    if a[i]>best{
      best:=a[i];
    }
    i:=i+1;
  }
  m:=best;
  i:=x;
  while i<a.Length
  {
    if a[i]>best{
      p:=i;
      return;
    }
    i:=i+1;
  }
  p:=a.Length-1;


}

