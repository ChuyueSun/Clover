method SwapArithmetic(X: int, Y: int) returns(x: int, y: int)
{
  x, y := X, Y;

  x := y - x;
  y := y - x;
  x := y + x;


}
