method SwapBitvectors(X: bv8, Y: bv8) returns(x: bv8, y: bv8)
  ensures x==Y^X
  ensures y==X
{
  x := X ^ Y;
  y := X;
}
