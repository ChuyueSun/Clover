method SwapSimultaneous(X: int, Y: int) returns(x: int, y: int)
  ensures x==Y
{
  x, y := X, Y;
  x, y := y, x;
}
