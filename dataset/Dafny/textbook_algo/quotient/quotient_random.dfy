method QZEM(a: nat, b:nat) returns (p:int, t:int)
  //TOFILL

{
  p:=a;
  t:=0;
  while b<=p
    invariant t*b+p==a && p>=0
    decreases p
  {
    p:=p-b;
    t:=t+1;

  }
}
