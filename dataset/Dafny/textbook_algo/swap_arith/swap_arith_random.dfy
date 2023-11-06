method ZWDX(AJKT: int, BVOP: int) returns(PKRG: int, QTSH: int)
  //TOFILL

{
  PKRG, QTSH := AJKT, BVOP;

  PKRG := QTSH - PKRG;
  QTSH := QTSH - PKRG;
  PKRG := QTSH + PKRG;

}
