method CalDiv() returns (x:int, y:int)
{
  x, y := 0, 191;
  while 7 <= y
    invariant 0 <= y
    invariant x > 0 ==> 7 * (x - 1) + y == 191
    invariant x == 0 ==> 7 * x + y == 191
  {
    y:=191-7*x;
    x := x+1;
  }
}
