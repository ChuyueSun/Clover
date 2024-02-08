method SwapArithmetic(X: int, Y: int) returns(x: int, y: int)
  requires X == Y
  ensures x==Y
  ensures y==X
{
  x, y := X, Y;
}
