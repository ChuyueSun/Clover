method onlineMax(a: array<int>, x: int) returns (ghost m:int, p:int)

{
  assert 1<=x<a.Length;
  assert a.Length!=0  ;

  p:= 0;
  var best := a[0];
  var i:=1;
  while i<x
    invariant 0<=i<=x
    invariant forall j::0<=j<i==> a[j]<=best
    invariant exists j::0<=j<i && a[j]==best
  {
    if a[i]>best{
      best:=a[i];
    }
    i:=i+1;
  }
  m:=best;
  i:=x;
  while i<a.Length
    invariant x<=i<=a.Length
    invariant forall j::x<=j<i ==> a[j]<=m
  {
    if a[i]>best{
      p:=i;
      return;
    }
    i:=i+1;
  }
  p:=a.Length-1;


}

