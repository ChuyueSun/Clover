method Swap(X: int, Y: int) returns(x: int, y: int)
  ensures x==X
  ensures y==Y
{
  x, y := X, Y;
}
