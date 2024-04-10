method SwapSimultaneous(X: int, Y: int) returns(x: int, y: int)
  ensures x==Y
  ensures y==X
{
  x, y := X, Y;
  x, y := y, x;
}
