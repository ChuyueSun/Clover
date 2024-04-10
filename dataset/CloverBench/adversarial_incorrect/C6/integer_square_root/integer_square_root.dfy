method SquareRoot(N:nat) returns (r:nat)
  ensures r > 0 ==> (r-1)*(r-1) < N <= r * r
  ensures r == 0 ==> N == 0
{
  r:=0;
  while r * r<N
    invariant r > 0 ==> (r-1) * (r-1) < N
  {
    r:=r+1;
  }
}
