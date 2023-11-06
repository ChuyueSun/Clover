method foo(x: int, y:int) returns (r:int, q:int)
  //TOFILL

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
