method foo(x: int) returns (a: int, b: int)
  ensures b == 2 * a
{
  a := 2 * x;
  b := 2 * a;
}
