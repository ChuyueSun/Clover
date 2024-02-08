method ZYXW() returns (ABCD:int, EFGH:int)
  //TOFILL
{

  ABCD, EFGH := 0, 191;
  while 7 <= EFGH
    invariant 0 <= EFGH && 7 * ABCD + EFGH == 191
  {
    ABCD := ABCD+1;
    EFGH:=191-7*ABCD;
  }
}
