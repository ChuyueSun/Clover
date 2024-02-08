method Quotient(x: nat, y:nat) returns (r:int, q:int)
  requires y != 0
  ensures q*y+r==x && q>=0
{
  r:=x;
  q:=0;
}
