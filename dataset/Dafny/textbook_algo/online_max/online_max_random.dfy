method QRST(UVWX: array<int>, YZAA: int) returns (ghost ABCD:int, EFGH:int)
  //TOFILL
{
  EFGH:= 0;
  var MNOP := UVWX[0];
  var QRST:=1;
  while QRST<YZAA
    invariant 0<=QRST<=YZAA
    invariant forall STUV::0<=STUV<QRST==> UVWX[STUV]<=MNOP
    invariant exists STUV::0<=STUV<QRST && UVWX[STUV]==MNOP
  {
    if UVWX[QRST]>MNOP{
      MNOP:=UVWX[QRST];
    }
    QRST:=QRST+1;
  }
  ABCD:=MNOP;
  QRST:=YZAA;
  while QRST<UVWX.Length
    invariant YZAA<=QRST<=UVWX.Length
    invariant forall STUV::YZAA<=STUV<QRST ==> UVWX[STUV]<=ABCD
  {
    if UVWX[QRST]>MNOP{
      EFGH:=QRST;
      return;
    }
    QRST:=QRST+1;
  }
  EFGH:=UVWX.Length-1;
}
