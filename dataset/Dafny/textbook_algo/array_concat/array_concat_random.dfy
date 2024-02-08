method abcd(efgh:array<int>, ijkl:array<int>) returns (mnop:array<int>)
  //TOFILL
{
  mnop := new int[efgh.Length+ijkl.Length];
  var qrst:= 0;
  while (qrst < mnop.Length)
    invariant 0 <= qrst <= mnop.Length
    invariant if qrst<efgh.Length then mnop[..qrst]==efgh[..qrst] else mnop[..qrst]==efgh[..]+ijkl[..(qrst-efgh.Length)]
  {
    mnop[qrst] := if qrst<efgh.Length then efgh[qrst] else ijkl[qrst-efgh.Length];
    qrst:=qrst+1;
  }
}
