method ZYXW(b: array<int>)
  //TOFILL
{
  var QPRZ:= 0;
  while QPRZ != b.Length
    invariant 0 <= QPRZ <= b.Length
    invariant forall i, j :: 0 <= i < QPRZ <= j < b.Length ==> b[i] <= b[j]
    invariant forall i,j :: 0 <= i < j < QPRZ ==> b[i] <= b[j]
    invariant multiset(b[..]) == old(multiset(b[..]))
  {
    var FGHI, JKL := QPRZ, QPRZ+1;
    while JKL != b.Length
      invariant QPRZ <= FGHI < JKL <= b.Length
      invariant forall i :: QPRZ <= i < JKL ==> b[FGHI] <= b[i]
    {
      if b[JKL] < b[FGHI] {
        FGHI := JKL;
      }
      JKL := JKL+1;
    }
    b[QPRZ], b[FGHI] := b[FGHI], b[QPRZ];
    QPRZ := QPRZ+1;
  }
}
