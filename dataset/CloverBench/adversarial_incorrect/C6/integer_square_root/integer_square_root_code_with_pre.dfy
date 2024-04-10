method SquareRoot(N:nat) returns (r:nat)

{
  r:=0;
  while r * r<N
    invariant r > 0 ==> (r-1) * (r-1) < N
  {
    r:=r+1;
  }
}
