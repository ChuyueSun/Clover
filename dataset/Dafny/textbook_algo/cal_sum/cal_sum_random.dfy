method XCDE() returns (QWIK:int, UFOT:int)
  //TOFILL
{
  UFOT, QWIK := 0, 0;
  while UFOT != 33
    invariant QWIK == UFOT * (UFOT + 1) / 2
  {
    UFOT := UFOT + 1;
    QWIK := QWIK + UFOT;
  }
}
