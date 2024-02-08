method ZYJK<T>(XTRZ: array<T>, WJDG: T -> bool) returns (KLBJ: int)
  //TOFILL
{
  KLBJ := 0;
  while true
    invariant 0 <= KLBJ < XTRZ.Length
    invariant exists i :: KLBJ <= i < XTRZ.Length && WJDG(XTRZ[i])
    decreases XTRZ.Length - KLBJ
  {
    if WJDG(XTRZ[KLBJ]) {
      return;
    }
    KLBJ := KLBJ + 1;
  }
}
