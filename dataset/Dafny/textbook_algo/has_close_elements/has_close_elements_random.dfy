method qXwZ(aBcD: seq<real>, eFgH: real) returns (iJkL: bool)
  //TOFILL
{
  iJkL := false;
  var uVwX: int := 0;
  while uVwX < |aBcD| && !iJkL
    invariant 0 <= uVwX <= |aBcD|
    invariant !iJkL
    invariant forall mNpO: int, qRsT: int :: 0 <= mNpO < uVwX && 0 <= qRsT < mNpO ==> (if aBcD[mNpO] - aBcD[qRsT] < 0.0 then aBcD[qRsT] - aBcD[mNpO] else aBcD[mNpO] - aBcD[qRsT]) >= eFgH
  {
    var yZaB: int := 0;
    while yZaB < uVwX && !iJkL
      invariant 0 <= uVwX <= |aBcD|
      invariant 0 <= yZaB <= uVwX
      invariant !iJkL
      invariant forall qRsT: int :: 0 <= qRsT < yZaB ==> (if aBcD[uVwX] - aBcD[qRsT] < 0.0 then aBcD[qRsT] - aBcD[uVwX] else aBcD[uVwX] - aBcD[qRsT]) >= eFgH
    {

      var cDeF := (if aBcD[yZaB] - aBcD[uVwX] < 0.0 then aBcD[uVwX] - aBcD[yZaB] else aBcD[yZaB] - aBcD[uVwX]);
      if cDeF < eFgH  {
        iJkL := true;
        return;
      }

      yZaB := yZaB + 1;
    }
    uVwX := uVwX + 1;
  }
}
