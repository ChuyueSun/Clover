method foo(X: int, Y: int) returns(x: int, y: int)
  //TOFILL

{
  x, y := X, Y;

  x := y - x;
  y := y - x;
  x := y + x;

}
