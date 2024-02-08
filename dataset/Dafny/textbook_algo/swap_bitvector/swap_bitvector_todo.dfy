method SwapBitvectors(X: bv8, Y: bv8) returns(x: bv8, y: bv8)
  //TOFILL
{
  x, y := X, Y;

  x := x ^ y;
  y := x ^ y;
  x := x ^ y;

}
