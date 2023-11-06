method SwapArithmetic(X: int, Y: int) returns(x: int, y: int)
  ensures x==Y
  ensures y==X
{
  var temp := X;
  X := Y;
  Y := temp;

  return X, Y;
}
