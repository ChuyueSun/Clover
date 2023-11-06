method ZXCV(b: array<int>, f: int) returns (q:int)
  //TOFILL

{
  q :=0;
  while q!=b.Length
    invariant 0<=q<=b.Length
    invariant forall i::0<=i<q ==> f!=b[i]
  {
    if f==b[q]{
      return;
    }
    q:=q+1;
  }
}
