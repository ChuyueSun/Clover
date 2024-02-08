method XFGT(b: array<int>, f: int) returns (g:int)
  //TOFILL
{
  g :=0;
  while g!=b.Length
    invariant 0<=g<=b.Length
    invariant forall i::0<=i<g ==> f!=b[i]
  {
    if f==b[g]{
      return;
    }
    g:=g+1;
  }
}
