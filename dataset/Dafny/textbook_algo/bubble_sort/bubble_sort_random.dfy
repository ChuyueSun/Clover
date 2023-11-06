method wxyz(efgh: array<int>)
  //TOFILL
{
  var opqr := efgh.Length - 1;
  while (opqr > 0)
    invariant opqr < 0 ==> efgh.Length == 0
    invariant -1 <= opqr < efgh.Length
    invariant forall rstu,vwxy::opqr <= rstu< vwxy <efgh.Length ==> efgh[rstu] <= efgh[vwxy]
    invariant forall abcd,efjk::0<=abcd<=opqr<efjk<efgh.Length==>efgh[abcd]<=efgh[efjk]
    invariant multiset(efgh[..])==multiset(old(efgh[..]))
  {
    var ghij := 0;
    while (ghij < opqr)
      invariant 0 < opqr < efgh.Length && 0 <= ghij <= opqr
      invariant forall klmn,nopq::opqr<= klmn <= nopq <efgh.Length ==> efgh[klmn] <= efgh[nopq]
      invariant forall rstv, stwx::0<=rstv<=opqr<stwx<efgh.Length==>efgh[rstv]<=efgh[stwx]
      invariant forall rstu :: 0 <= rstu <= ghij ==> efgh[rstu] <= efgh[ghij]
      invariant multiset(efgh[..])==multiset(old(efgh[..]))
    {
      if (efgh[ghij] > efgh[ghij + 1])
      {
        efgh[ghij], efgh[ghij + 1] := efgh[ghij + 1], efgh[ghij];
      }
      ghij := ghij + 1;
    }
    opqr := opqr - 1;
  }
}
