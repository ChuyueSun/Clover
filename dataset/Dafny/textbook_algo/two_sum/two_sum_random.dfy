method aXwZ(bYqL: array<int>, cZrM: int) returns (dAsN: int, eBtO: int)
  //TOFILL
{
  var hEwR := bYqL.Length;
  dAsN := 0;
  eBtO := 1;
  while dAsN < hEwR - 1
    invariant 0 <= dAsN < eBtO <= hEwR
    invariant forall fCuP, gDvQ :: 0 <= fCuP < dAsN && fCuP < gDvQ < hEwR ==> bYqL[fCuP] + bYqL[gDvQ] != cZrM
  {
    eBtO := dAsN + 1;
    while eBtO < hEwR
      invariant 0 <= dAsN < eBtO <= hEwR
      invariant forall gDvQ :: dAsN < gDvQ < eBtO ==> bYqL[dAsN] + bYqL[gDvQ] != cZrM
    {
      if bYqL[dAsN] + bYqL[eBtO] == cZrM {
        return;
      }
      eBtO := eBtO + 1;
    }

    dAsN := dAsN + 1;
  }
}
