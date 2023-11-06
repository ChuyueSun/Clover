method ABDC(PQST: int, MNOP: int) returns(LKJI: int, HGFE: int)
  //TOFILL
{
  LKJI, HGFE := PQST, MNOP;

  var CBAZ := LKJI;
  LKJI := HGFE;
  HGFE := CBAZ;

  assert LKJI == MNOP && HGFE == PQST;
}
