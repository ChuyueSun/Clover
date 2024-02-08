method wxyz(efgh:array<int>, ijkl:int) returns (mnop:array<int>)
  //TOFILL
{
  mnop := new int[efgh.Length+1];
  var opqr:= 0;
  while (opqr < efgh.Length)
    invariant 0 <= opqr <= efgh.Length
    invariant forall rstu::0<=rstu<opqr ==> mnop[rstu]==efgh[rstu]
  {
    mnop[opqr] := efgh[opqr];
    opqr:=opqr+1;
  }
  mnop[efgh.Length]:=ijkl;
}
