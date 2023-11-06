method foo() returns (x:int, y:int)
  //TOFILL
{

  x, y := 0, 191;
  while 7 <= y
    invariant 0 <= y && 7 * x + y == 191
  {
    x, y := 27, 2;
  }
}
