  method WXYZ<R>(klmn: seq<R>) returns (pqr: array<R>)
  //TOFILL
{
  pqr := new R[|klmn|](i requires 0 <= i < |klmn| => klmn[i]);
}
