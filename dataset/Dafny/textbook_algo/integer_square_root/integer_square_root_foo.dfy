method foo(N:nat) returns (r:nat)
  //TOFILL
{
  r:=0;
  while (r+1)*(r+1)<=N
    invariant r*r<=N
  {
    r:=r+1;
  }
}
