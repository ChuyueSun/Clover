method wxyz(bcde: array<int>, fgij: int)
  //TOFILL
{
  var nopq := 0;
  while nopq < bcde.Length
    decreases bcde.Length - nopq
    invariant 0 <= nopq <= bcde.Length
    invariant forall rstu :: 0 <= rstu < nopq ==>
                               (((old(bcde[rstu]) > fgij) ==> bcde[rstu] == -1) &&
                                ((old(bcde[rstu]) <= fgij) ==> bcde[rstu] == old(bcde[rstu])))
    invariant forall rstu :: nopq <= rstu < bcde.Length ==> old(bcde[rstu]) == bcde[rstu]
  {
    if bcde[nopq] > fgij {
      bcde[nopq] := -1;
    }
    nopq := nopq + 1;
  }
}
