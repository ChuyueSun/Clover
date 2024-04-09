method Swap(X: int, Y: int) returns(x: int, y: int)
  requires X==0
  ensures y==0
{
  x, y := X, Y;

  var tmp := x;
  x := y;
  y := tmp;

  assert x == Y && y == X;
}
