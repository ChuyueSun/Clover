method ZYXW(ABBA: array<int>, BCDC: array<int>) returns (DEFA: array<int> )
  //TOFILL
{
  DEFA:= new int[ABBA.Length];
  var GHJK:=0;
  while GHJK<ABBA.Length
    invariant 0<=GHJK<=ABBA.Length
    invariant forall MNOP:: 0 <= MNOP< GHJK==> ABBA[MNOP] * BCDC[MNOP]==DEFA[MNOP]
  {
    DEFA[GHJK]:=ABBA[GHJK]*BCDC[GHJK];
    GHJK:=GHJK+1;
  }
}
