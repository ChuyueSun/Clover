method SwapBitvectors(X: bv8, Y: bv8) returns(x: bv8, y: bv8)

{
  x := X ^ Y;
  y := X;
}
