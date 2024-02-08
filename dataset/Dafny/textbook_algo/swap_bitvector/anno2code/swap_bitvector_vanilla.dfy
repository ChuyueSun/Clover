method SwapBitvectors(X: bv8, Y: bv8) returns(x: bv8, y: bv8)
  ensures x == Y
  ensures y == X
{
  var temp: bv8;
  temp := X;
  X := Y;
  Y := temp;
  return (X, Y);
}
