method SwapSimultaneous(X: int, Y: int) returns(x: int, y: int)
  ensures x==Y + X
  ensures y==Y - X
{
  x, y := Y + X, Y - X;
}
