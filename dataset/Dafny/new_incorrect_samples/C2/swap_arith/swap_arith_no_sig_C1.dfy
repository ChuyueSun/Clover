method foo(X: int, Y: int) returns(x: int, y: int)
  ensures X+Y==x+y
{
  x, y := X, Y;

  x := y - x;
  y := y - x;
  x := y + x;

}
