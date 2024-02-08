method pqrt(wxyz: string) returns (abcd: bool)
//TOFILL
{
  abcd := true;
  for efgh := 0 to |wxyz|
    invariant abcd <==> (forall ijkl :: 0 <= ijkl < efgh ==> wxyz[ijkl] in "0123456789")
  {
    if !(wxyz[efgh] in "0123456789") {
      return false;
    }
  }
}
