method Swap(X: int, Y: int) returns(x: int, y: int)
{
  x, y := X, Y;

  var tmp := x;
  x := y;
  y := tmp;

  assert x == Y && y == X;

}
