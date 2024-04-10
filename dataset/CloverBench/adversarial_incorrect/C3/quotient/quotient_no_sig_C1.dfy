method foo(x: nat, y:nat) returns (r:int, q:int)
  requires y != 0
  ensures q*y+r==x
{
  r:=x;
  q:=0;
  while y<=r
    invariant q*y+r==x && r>=0
    decreases r
  {
    r:=r-y;
    q:=q+1;

  }
}
