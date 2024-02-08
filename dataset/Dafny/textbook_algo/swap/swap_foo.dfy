method foo(X: int, Y: int) returns(x: int, y: int)
  //TOFILL
{
  x, y := X, Y;

  var tmp := x;
  x := y;
  y := tmp;

  assert x == Y && y == X;
}
