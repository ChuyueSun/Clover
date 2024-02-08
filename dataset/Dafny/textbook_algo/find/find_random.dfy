method ZYXW(b: array<int>, klmn: int) returns (ijkl: int)
  //TOFILL
{
  ijkl := 0;
  while ijkl < b.Length
    invariant 0<=ijkl<=b.Length
    invariant forall i :: 0 <= i < ijkl ==> b[i] != klmn
  {
    if b[ijkl] == klmn { return; }
    ijkl := ijkl + 1;
  }
  if ijkl >= b.Length  {
    ijkl := -1;
  }
}
