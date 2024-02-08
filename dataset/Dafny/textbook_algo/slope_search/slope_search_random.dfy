method ABDE(FEGH: array2<int>, IJKL: int) returns (MNOP:int, QRST:int)
  //TOFILL
{
  MNOP,QRST:=0, FEGH.Length1-1;
  while FEGH[MNOP,QRST] !=IJKL
    invariant 0<=MNOP<FEGH.Length0 && 0<=QRST<FEGH.Length1
    invariant exists UVWX,YZAB :: MNOP<=UVWX<FEGH.Length0 && 0<=YZAB<=QRST && FEGH[UVWX,YZAB]==IJKL
    decreases FEGH.Length0-MNOP+QRST
  {
    if FEGH[MNOP,QRST] < IJKL {
      MNOP:=MNOP+1;

    }else{
      QRST:=QRST-1;
    }
  }
}
