method Quotient(x: nat, y:nat) returns (r:int, q:int)
{
  r:=x;
  q:=0;
  while y<=r
  {
    r:=r-y;
    q:=q+1;

  }
}
