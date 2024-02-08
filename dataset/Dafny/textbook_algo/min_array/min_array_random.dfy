method XJQZ(BSTO: array<int>) returns (WONU:int)
  //TOFILL
{
  WONU:=BSTO[0];
  var RJDE:=1;
  while RJDE<BSTO.Length
    invariant 0 <= RJDE <= BSTO.Length
    invariant forall MYPQ :: 0 <= MYPQ < RJDE ==> WONU <= BSTO[MYPQ]
    invariant exists MYPQ :: 0 <= MYPQ < RJDE && WONU == BSTO[MYPQ]
  {
    if WONU>BSTO[RJDE]{
      WONU:=BSTO[RJDE];
    }
    RJDE:=RJDE+1;
  }
}
