method DoubleQuadruple(x: int) returns (a: int, b: int)
  ensures b == 4 * x
{
  a := x;
  b := 4 * a;
}
